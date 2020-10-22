use super::{Block, Chunk, ChunkPosition};
use crate::actor_model::*;
use crate::helpers::Vec3d;
use crate::packet::play::send::{ChunkData, UpdateLight};
use crate::packet::PlayerConnectionPacketHandle;
use async_trait::async_trait;
use std::cmp;
use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::{broadcast, mpsc, oneshot};

// TODO: Send update packets to subscribers

/// Width of a region in chunks. One region
/// contains `x * x` chunks where `x` is this
/// constant.
pub const REGION_CHUNK_WIDTH: u32 = 2;

/// The factor to multiply a region position with to get
/// the lowest block position inside the chunk.
/// This is the width of a region in blocks.
const REGION_COORDINATE_SCALE: i32 = (REGION_CHUNK_WIDTH * super::CHUNK_BLOCK_WIDTH) as i32;

const CHUNK_ARRAY_LEN: usize = (REGION_CHUNK_WIDTH * REGION_CHUNK_WIDTH) as usize;

/// A region contains the chunks and is responsible to
/// actions regarding to blocks.
pub struct Region {
  pub position: RegionPosition,
  pub chunks: Box<[Option<Chunk>; CHUNK_ARRAY_LEN]>,
  /// Subscribed online players receive block / chunk packets
  /// when a chunk changes.
  pub subscribers: [HashMap<u32, PlayerConnectionPacketHandle>; CHUNK_ARRAY_LEN],
  handle: Option<<Region as Actor>::Handle>,
  ticks_to_process: Arc<Mutex<u8>>,
}

impl Region {
  pub fn new(pos: RegionPosition, chunks: Box<[Option<Chunk>; CHUNK_ARRAY_LEN]>) -> Self {
    Self {
      position: pos,
      chunks,
      subscribers: [
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
      ],
      handle: None,
      ticks_to_process: Mutex::new(0u8).into(),
    }
  }
  /// Takes an absolute chunk position and converts it
  /// into an index for the chunks array. This assumes
  /// that the position is not out of bounds.
  fn get_chunk_index(&self, pos: &ChunkPosition) -> usize {
    let chk_off: ChunkPosition = *pos - self.position.get_min_chunk_pos();
    debug_assert!(
      self.has_chunk(&pos),
      "Chunk at position {:?} is not in this region at {:?}",
      pos,
      self.position
    );
    Self::get_chunk_index_from_offset(&chk_off)
  }
  /// Takes a relative chunk offset and converts it
  /// into an index for the chunks array. This assumes
  /// that the offset is not out of bounds.
  fn get_chunk_index_from_offset(off: &ChunkPosition) -> usize {
    debug_assert!(Self::has_chunk_from_offset(&off));
    let idx = (off.x + REGION_CHUNK_WIDTH as i32 * off.z) as usize;
    debug_assert!(
      (0..CHUNK_ARRAY_LEN).contains(&idx),
      "Index {idx} from offset {pos:?} is out of bounds 0..{bounds}",
      idx = idx,
      pos = off,
      bounds = CHUNK_ARRAY_LEN
    );
    idx
  }
  pub fn get_chunk(&self, pos: ChunkPosition) -> Option<&Chunk> {
    if self.has_chunk(&pos) {
      self.chunks[self.get_chunk_index(&pos)].as_ref()
    } else {
      None
    }
  }
  /// Returns the chunk (if loaded) that is at the given offset.
  /// Returns None if the position is out of bounds.
  pub fn get_chunk_at_offset(&self, offset: ChunkPosition) -> Option<&Chunk> {
    if Self::has_chunk_from_offset(&offset) {
      self.chunks[Self::get_chunk_index_from_offset(&offset)].as_ref()
    } else {
      None
    }
  }
  /// Returns the mutable chunk (if loaded) that contains the given offset
  pub fn get_chunk_at_offset_mut(
    &mut self,
    offset: impl Into<ChunkPosition>,
  ) -> Option<&mut Chunk> {
    let offset = offset.into();
    if self.has_chunk(&offset) {
      self.chunks[Self::get_chunk_index_from_offset(&offset)].as_mut()
    } else {
      None
    }
  }
  /// Checks whether the given absolute chunk position is inside the bounds
  /// of this region.
  pub fn has_chunk(&self, pos: &ChunkPosition) -> bool {
    let chk_off: ChunkPosition = *pos - self.position.get_min_chunk_pos();
    Self::has_chunk_from_offset(&chk_off)
  }
  /// Checks whether the given absolute chunk position is inside the bounds
  /// of this region.
  pub fn has_chunk_from_offset(off: &ChunkPosition) -> bool {
    off.x >= 0
      && off.z >= 0
      && off.x < REGION_CHUNK_WIDTH as i32
      && off.z < REGION_CHUNK_WIDTH as i32
  }
  pub fn set_chunk(&mut self, chunk: Box<Chunk>) {
    if self.has_chunk(&chunk.position) {
      let idx = self.get_chunk_index(&chunk.position);
      self.chunks[idx] = Some(*chunk);
    } else {
      panic!(
        "Attempt to set chunk in region that does not store it. Chunk: {:?}, Region: {:?}",
        chunk.position, self.position
      );
    }
  }
  /// Process one game tick on this region
  pub fn tick(&mut self) {
    // TODO
  }
}

#[derive(Debug, cmp::PartialEq, cmp::Eq, Clone, Copy)]
pub struct RegionPosition {
  pub x: i32,
  pub z: i32,
}
impl RegionPosition {
  pub fn new(x: i32, z: i32) -> Self {
    Self { x, z }
  }
  pub fn get_offset(&self) -> Vec3d<i32> {
    Vec3d::new(
      self.x * REGION_COORDINATE_SCALE,
      0,
      self.z * REGION_COORDINATE_SCALE,
    )
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
      x: vec.x >> 5,
      z: vec.z >> 5,
    }
  }
}
impl From<ChunkPosition> for RegionPosition {
  fn from(pos: ChunkPosition) -> Self {
    Self {
      x: pos.x >> 1,
      z: pos.z >> 1,
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
    mut recv: mpsc::Receiver<ActorMessage<<Self::Handle as ActorHandle>::Message>>,
  ) -> Self {
    use std::time::Duration;
    {
      // Send a message to the message channel 20 times a second
      // To tell the region to process a game tick. If the region
      // has too much to calculate, the interval channel fills up
      // until it is full.

      // Store how many ticks this region still has to process
      let ticks_to_process = self.ticks_to_process.clone();
      // Store how many ticks this region is lagging behind
      let mut ticks_handle = self.clone_handle();
      // This thread is going to sleep most of the time, so green
      // threads should be used.
      tokio::task::spawn(async move {
        let mut intv = tokio::time::interval(Duration::from_secs_f32(1f32 / 20f32));
        let mut lag: u16 = 0;
        loop {
          intv.tick().await;
          let mut ttp = ticks_to_process.lock().await;
          match *ttp {
            0..=3 if lag > 0 => {
              // Process two ticks if lagging behind
              if let Err(()) = ticks_handle.perform_tick(2).await {
                eprintln!("[region.rs] Failed to send tick message");
                return;
              }
              lag -= 1;
              *ttp += 2;
            }
            0..=19 => {
              // Process one tick
              if let Err(()) = ticks_handle.perform_tick(1).await {
                eprintln!("[region.rs] Failed to send tick message");
                return;
              }
              *ttp += 1;
            }
            20..=255 => {
              // Skipping ticks if the region is lagging and
              // remember how many ticks were skipped
              lag = lag.saturating_add(1);
            }
          }
        }
      });
    };
    // This loop handles all incoming messages before processing the next tick
    loop {
      // Try to read a message
      match recv.recv().await {
        Some(msg) => {
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
        None => {
          eprintln!("All handles for Actor 🧍 '{}' were dropped", self);
          return self;
        }
      }
    }
  }

  async fn handle_message(&mut self, message: <Self::Handle as ActorHandle>::Message) -> bool {
    match message {
      RegionMessage::PerformTick(amount) => {
        for _ in 0..amount {
          self.tick();
          let mut ttp = self.ticks_to_process.lock().await;
          *ttp = ttp.saturating_sub(1);
        }
        true
      }
      // Process the message
      RegionMessage::GetBlock {
        offset: off,
        channel: send,
      } => {
        if (0..REGION_COORDINATE_SCALE).contains(&off.x)
          && (0..256).contains(&off.y)
          && (0..REGION_COORDINATE_SCALE).contains(&off.z)
        {
          if let Some(chunk) = self.get_chunk_at_offset(off.into()) {
            let chk_off = (Into::<ChunkPosition>::into(off.clone())).get_offset();
            let inner_offset = off - chk_off;
            send_block(
              send,
              Some(chunk.get_block_at_pos(Vec3d::new(
                inner_offset.x as u8,
                inner_offset.y as u8,
                inner_offset.z as u8,
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
        if self.has_chunk(&chunk.position) {
          self.set_chunk(chunk);
        } else {
          eprintln!(
            "[region.rs] Attempted to set chunk in this region that doesn't exist in this region"
          );
        }
        return true;
      }
      #[allow(deprecated)]
      RegionMessage::BroadcastChunk { offset, sender } => {
        let chunk = match self.get_chunk_at_offset(offset) {
          Some(c) => c,
          None => {
            eprintln!("Failed to send chunk packet in RegionMessage::BroadcastChunk because the requested chunk is not in this region");
            send_chunk(sender, None);
            return true;
          }
        };
        let packet = ChunkData::from_chunk(&chunk);
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
              eprintln!("[region.rs] Failed to send chunk packet in RegionMessage::BroadcastChunk because all receivers were dropped");
            }
          }
        }
      }
      RegionMessage::PlayerSubscribe {
        chunk: position,
        player_id,
        mut connection,
        send_complete,
        callback,
      } => {
        if self.has_chunk(&position) {
          if send_complete {
            if let Some(chunk) = self.get_chunk(position) {
              if let Err(e) = (async {
                let packet = UpdateLight::from_chunk(chunk, true);
                connection.send_packet(packet).await?;
                let packet = ChunkData::from_chunk(chunk);
                connection.send_packet(packet).await?;
                Ok(())
              })
              .await
              {
                // Enforce error type
                let e: String = e;
                eprintln!("[region.rs] Failed to send chunk packets: {}", e);
              }
            } else {
              eprintln!("[region.rs] Attempt to request unloaded chunk");
            }
          }
          self.subscribers[self.get_chunk_index(&position)].insert(player_id, connection);
          println!("DEBUG Sending callback");
          if let Err(_) = callback.send(self.clone_handle()) {
            eprintln!("[region.rs] Failed to send callback");
          } else {
            println!("DEBUG Sent callback");
          }
        } else {
          eprintln!("[region.rs] Attempt to subscribe to chunk in wrong region");
        }
        true
      }
      RegionMessage::PlayerUnsubscribe { chunk, player_id } => {
        if self.has_chunk(&chunk) {
          self.subscribers[self.get_chunk_index(&chunk)].remove(&player_id);
        }
        true
      }
    }
  }

  fn create_handle(
    &self,
    sender: mpsc::Sender<ActorMessage<<Self::Handle as ActorHandle>::Message>>,
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
  /// Tells the region to perform one game tick
  async fn perform_tick(&mut self, amount: u8) -> Result<(), ()> {
    self
      .send_raw_message(ActorMessage::Other(RegionMessage::PerformTick(amount)))
      .await
      .map_err(|_| ())
  }
  /// Get a block inside of this region with the given offset.
  ///
  /// Note: Do not use absolute block coordinates.
  pub async fn get_block(&mut self, offset: Vec3d<i32>) -> Result<Option<Block>, ()> {
    let (sender, callback) = oneshot::channel();
    self
      .send_raw_message(ActorMessage::Other(RegionMessage::GetBlock {
        offset,
        channel: sender,
      }))
      .await
      .map_err(|_| ())?;
    callback.await.map_err(|_| ())
  }

  pub async fn set_chunk(&mut self, chunk: Box<Chunk>) -> Result<(), ()> {
    self
      .send_raw_message(ActorMessage::Other(RegionMessage::SetChunk(chunk)))
      .await
      .map_err(|_| ())
  }

  #[deprecated(since = "0.1.0", note = "Use the subscription model!")]
  pub async fn broadcast_chunk(
    &mut self,
    offset: ChunkPosition,
    sender: broadcast::Sender<Option<Arc<(u32, Vec<u8>)>>>,
  ) -> Result<(), ()> {
    self
      .send_raw_message(ActorMessage::Other(
        #[allow(deprecated)]
        RegionMessage::BroadcastChunk { offset, sender },
      ))
      .await
      .map_err(|_| ())
  }

  pub async fn player_subscribe(
    &mut self,
    chunk: ChunkPosition,
    player_id: u32,
    connection: PlayerConnectionPacketHandle,
    send_complete: bool,
    callback: oneshot::Sender<RegionHandle>,
  ) -> Result<(), ()> {
    self
      .send_raw_message(ActorMessage::Other(RegionMessage::PlayerSubscribe {
        chunk,
        player_id,
        connection,
        send_complete,
        callback,
      }))
      .await
      .map_err(|_| ())
  }

  pub async fn player_unsubscribe(
    &mut self,
    chunk: ChunkPosition,
    player_id: u32,
  ) -> Result<(), ()> {
    self
      .send_raw_message(ActorMessage::Other(RegionMessage::PlayerUnsubscribe {
        chunk,
        player_id,
      }))
      .await
      .map_err(|_| ())
  }
}

#[derive(Debug)]
pub enum RegionMessage {
  /// Get a block if it is stored by this region and loaded
  ///
  ///
  GetBlock {
    offset: Vec3d<i32>,
    channel: oneshot::Sender<Option<Block>>,
  },
  SetChunk(Box<Chunk>),
  /// Broadcast a chunk
  #[deprecated(since = "0.1.0", note = "Use the subscription model!")]
  BroadcastChunk {
    offset: ChunkPosition,
    sender: broadcast::Sender<Option<Arc<(u32, Vec<u8>)>>>,
  },
  PlayerSubscribe {
    chunk: ChunkPosition,
    player_id: u32,
    connection: PlayerConnectionPacketHandle,
    /// If the chunk should be sent as a complete
    /// chunk first
    send_complete: bool,
    callback: oneshot::Sender<RegionHandle>,
  },
  PlayerUnsubscribe {
    chunk: ChunkPosition,
    player_id: u32,
  },
  PerformTick(u8),
}
