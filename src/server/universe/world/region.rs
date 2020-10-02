use super::{Block, Chunk, ChunkPosition};
use crate::actor_model::*;
use crate::helpers::Vec3d;
use crate::packet::play::send::{ChunkData, ChunkSectionData};
use async_trait::async_trait;
use std::cmp;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use tokio::sync::mpsc::{channel, error::SendError, Receiver, Sender};
use tokio::sync::{broadcast, oneshot};

pub const REGION_CHUNK_WIDTH: u32 = 2;
const COORDINATE_SCALE: i32 = (REGION_CHUNK_WIDTH * super::CHUNK_BLOCK_WIDTH) as i32;

pub struct Region {
  position: RegionPosition,
  chunks: [Option<Chunk>; (REGION_CHUNK_WIDTH * REGION_CHUNK_WIDTH) as usize],
  handle: Option<<Region as Actor>::Handle>,
}

impl Region {
  fn get_chunk_index(&self, pos: &ChunkPosition) -> isize {
    let chk_off: ChunkPosition = *pos - self.position.get_min_chunk_pos();
    (chk_off.get_x() + REGION_CHUNK_WIDTH as i32 * chk_off.get_z()) as isize
  }
  /// Returns the chunk (if loaded) that contains the given offset
  pub fn get_chunk_containing_offset(&self, offset: impl Into<ChunkPosition>) -> Option<&Chunk> {
    // Should be a number between 0 and 3 (inclusive), otherwise it is out of bounds
    let i = self.get_chunk_index(&offset.into());
    if (0..self.chunks.len() as isize).contains(&i) {
      self.chunks[i as usize].as_ref()
    } else {
      None
    }
  }
  /// Returns the mutable chunk (if loaded) that contains the given offset
  pub fn get_chunk_containing_offset_mut(
    &mut self,
    offset: impl Into<ChunkPosition>,
  ) -> Option<&mut Chunk> {
    // Should be a number between 0 and 3 (inclusive), otherwise it is out of bounds
    let i = self.get_chunk_index(&offset.into());
    if (0..self.chunks.len() as isize).contains(&i) {
      self.chunks[i as usize].as_mut()
    } else {
      None
    }
  }
  pub fn has_chunk(&self, pos: &ChunkPosition) -> bool {
    let chk_off: ChunkPosition = *pos - self.position.get_min_chunk_pos();
    (0..REGION_CHUNK_WIDTH as i32).contains(&chk_off.x)
      && (0..REGION_CHUNK_WIDTH as i32).contains(&chk_off.z)
  }
  pub fn set_chunk(&mut self, chunk: Chunk) {
    // Should be a number between 0 and 3 (inclusive), otherwise it is out of bounds
    let i = self.get_chunk_index(&chunk.get_position());
    if (0..self.chunks.len() as isize).contains(&i) {
      self.chunks[i as usize] = Some(chunk);
    } else {
      panic!("Attempt to set chunk in region that does not store it");
    }
  }
  /// Process one game tick on this region
  pub fn tick(&mut self) {
    // TODO
  }
}

#[derive(Debug, cmp::PartialEq, cmp::Eq)]
pub struct RegionPosition {
  x: i32,
  z: i32,
}
impl RegionPosition {
  pub fn new(x: i32, z: i32) -> Self {
    Self { x, z }
  }
  pub fn get_x(&self) -> i32 {
    self.x
  }
  pub fn get_z(&self) -> i32 {
    self.z
  }
  pub fn get_offset(&self) -> Vec3d<i32> {
    Vec3d::new(self.x * COORDINATE_SCALE, 0, self.z * COORDINATE_SCALE)
  }
  /// Returns the minimum chunk position included in this area
  pub fn get_min_chunk_pos(&self) -> ChunkPosition {
    ChunkPosition::new(
      self.x * REGION_CHUNK_WIDTH as i32,
      self.z * REGION_CHUNK_WIDTH as i32,
    )
  }
  /// Returns the maximum chunk position included in this area
  pub fn get_max_chunk_pos(&self) -> ChunkPosition {
    ChunkPosition::new(
      (self.x + 1) * REGION_CHUNK_WIDTH as i32 - 1,
      (self.z + 1) * REGION_CHUNK_WIDTH as i32 - 1,
    )
  }
}
impl From<Vec3d<i32>> for RegionPosition {
  fn from(vec: Vec3d<i32>) -> Self {
    Self {
      x: vec.get_x() / COORDINATE_SCALE,
      z: vec.get_z() / COORDINATE_SCALE,
    }
  }
}

impl Hash for RegionPosition {
  fn hash<H: Hasher>(&self, state: &mut H) {
    state.write_i32(self.x);
    state.write_i32(self.z);
  }
}

impl fmt::Display for RegionPosition {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "({}, {})", self.x, self.z)
  }
}

impl fmt::Display for Region {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Region {}", self.position)
  }
}

pub type RegionHandle = ActorHandleStruct<RegionMessage>;

#[async_trait]
impl Actor for Region {
  type Handle = RegionHandle;

  async fn start_actor(
    mut self,
    mut recv: Receiver<ActorMessage<<Self::Handle as ActorHandle>::Message>>,
  ) -> Self {
    use std::time::Duration;
    use tokio::sync::mpsc::error::{TryRecvError, TrySendError};
    let mut tick_interval = {
      // Send a message to the interval channel 20 times a second
      // To tell the region to process a game tick. If the region
      // has too much to calculate, the interval channel fills up
      // until it is full.
      let (mut s, r) = channel(20);
      // This thread is going to sleep most of the time, so green
      // threads should be used.
      tokio::task::spawn(async move {
        let mut intv = tokio::time::interval(Duration::from_secs_f32(1f32 / 20f32));
        loop {
          intv.tick().await;
          match s.try_send(()) {
            Ok(()) => (),
            Err(TrySendError::Full(_)) => {
              continue;
            }
            Err(TrySendError::Closed(_)) => {
              return;
            }
          }
        }
      });
      r
    };
    // This loop handles all incoming messages before processing the next tick
    loop {
      // Try to read a message
      match recv.try_recv() {
        Ok(msg) => {
          match msg {
            ActorMessage::StopActor => {
              return self;
            }
            ActorMessage::Other(msg) => {
              if !self.handle_message(msg).await {
                return self;
              }
            }
          }
          continue;
        }
        Err(TryRecvError::Empty) => {
          // If no more messages are available, process the next tick if necessary
          // Ticks only fire every 1/20 seconds or more (depending on delay caused by messages)
          // If multiple ticks are missed, they stacked on the tick channel
          match tick_interval.try_recv() {
            Ok(()) => {
              // Do a tick
              self.tick();
            }
            Err(TryRecvError::Empty) => {
              continue;
            }
            Err(TryRecvError::Closed) => {
              return self;
            }
          }
          continue;
        }
        Err(TryRecvError::Closed) => {
          eprintln!("All handles for Actor 🧍 '{}' were dropped", self);
          return self;
        }
      }
    }
  }

  async fn handle_message(&mut self, message: <Self::Handle as ActorHandle>::Message) -> bool {
    match message {
      // Process the message
      RegionMessage::GetBlock {
        offset: off,
        channel: send,
      } => {
        if (0..COORDINATE_SCALE).contains(off.get_x_as_ref())
          && (0..256).contains(off.get_y_as_ref())
          && (0..COORDINATE_SCALE).contains(off.get_z_as_ref())
        {
          if let Some(chunk) = self.get_chunk_containing_offset(off.clone()) {
            let chk_off = (Into::<ChunkPosition>::into(off.clone())).get_offset();
            let inner_offset = off - chk_off;
            send_block(
              send,
              Some(chunk.get_block_at_pos(Vec3d::new(
                inner_offset.get_x() as u8,
                inner_offset.get_y() as u8,
                inner_offset.get_z() as u8,
              ))),
            );
            return true;
          }
        }
        send_block(send, None);
        return true;

        #[inline]
        fn send_block(send: oneshot::Sender<Option<Block>>, b: Option<Block>) {
          match send.send(b) {
            Ok(()) => (),
            Err(_) => {
              eprintln!("Failed to send block in RegionMessage::GetBlock");
            }
          }
        }
      }
      RegionMessage::SetChunk(chunk) => {
        if self.has_chunk(&chunk.get_position()) {
          self.set_chunk(chunk);
        } else {
          eprintln!(
            "[region.rs] Attempted to set chunk in this region that doesn't exist in this region"
          );
        }
        return true;
      }
      RegionMessage::BroadcastChunk { offset, sender } => {
        use crate::helpers::Registry;
        let chunk = match self.get_chunk_containing_offset(offset) {
          Some(c) => c,
          None => {
            eprintln!("Failed to send chunk packet in RegionMessage::BroadcastChunk because the requested chunk is not in this region");
            send_chunk(sender, None);
            return true;
          }
        };
        let mut sections_data: [_; 16] = [
          None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
          None,
        ];
        for i in 0..16 {
          if let Some(section) = &chunk.sections[i] {
            sections_data[i] = Some(ChunkSectionData::from_section(section));
          }
        }
        let packet = ChunkData {
          chunk_position: self.position.get_min_chunk_pos() + offset,
          heightmaps: Default::default(),
          biomes: Some([crate::server::registries::Biome::SnowyBeach.get_id() as u32; 1024]),
          sections: sections_data,
          block_entities: Vec::new(),
        };
        send_chunk(sender, Some(packet));
        return true;

        #[inline]
        fn send_chunk(send: broadcast::Sender<Option<Arc<(u32, Vec<u8>)>>>, b: Option<ChunkData>) {
          let data = if let Some(packet) = b {
            use crate::packet::PacketSerialOut;
            let mut buf = Vec::with_capacity(100);
            packet
              .consume_write(&mut buf)
              .expect("[region.rs] Failed to write chunk packet into buffer");
            Some(Arc::new((ChunkData::ID, buf)))
          } else {
            None
          };
          match send.send(data) {
            Ok(_) => (),
            Err(_) => {
              eprintln!("Failed to send chunk packet in RegionMessage::BroadcastChunk because all receivers were dropped");
            }
          }
        }
      }
    }
  }

  fn create_handle(
    &self,
    sender: Sender<ActorMessage<<Self::Handle as ActorHandle>::Message>>,
  ) -> Self::Handle {
    sender.into()
  }

  fn set_handle(&mut self, handle: Self::Handle) {
    self.handle = Some(handle);
  }

  fn clone_handle(&self) -> Self::Handle {
    self.handle.as_ref().unwrap().clone()
  }
}

impl RegionHandle {
  /// Get a block inside of this region with the given offset
  pub async fn get_block(
    &mut self,
    offset: Vec3d<i32>,
  ) -> Result<Option<Block>, SendError<ActorMessage<RegionMessage>>> {
    let (sender, callback) = oneshot::channel();
    self
      .send_raw_message(ActorMessage::Other(RegionMessage::GetBlock {
        offset,
        channel: sender,
      }))
      .await?;
    match callback.await {
      Ok(opt) => Ok(opt),
      Err(_) => panic!("The sender channel got dropped somehow"),
    }
  }

  pub async fn set_chunk(
    &mut self,
    chunk: Chunk,
  ) -> Result<(), SendError<ActorMessage<RegionMessage>>> {
    self
      .send_raw_message(ActorMessage::Other(RegionMessage::SetChunk(chunk)))
      .await
  }

  pub async fn broadcast_chunk(
    &mut self,
    offset: ChunkPosition,
    sender: broadcast::Sender<Option<Arc<(u32, Vec<u8>)>>>,
  ) -> Result<(), SendError<ActorMessage<RegionMessage>>> {
    self
      .send_raw_message(ActorMessage::Other(RegionMessage::BroadcastChunk {
        offset,
        sender,
      }))
      .await
  }
}

#[derive(Debug)]
pub enum RegionMessage {
  /// Get a block using a "fishing rod" by sending a callback sender
  GetBlock {
    offset: Vec3d<i32>,
    channel: oneshot::Sender<Option<Block>>,
  },
  SetChunk(Chunk),
  /// Broadcast a chunk
  BroadcastChunk {
    offset: ChunkPosition,
    sender: broadcast::Sender<Option<Arc<(u32, Vec<u8>)>>>,
  },
}
