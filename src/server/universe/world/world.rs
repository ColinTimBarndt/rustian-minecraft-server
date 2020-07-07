use super::chunk_generator::ChunkGenerator;
use super::chunk_loader::ChunkLoader;
use super::region::*;
use super::Block;
use super::EntityList;
use crate::actor_model::*;
use crate::helpers::{NamespacedKey, Vec3d};
use async_trait::async_trait;
use std::collections::{HashMap, HashSet};
use std::fmt;
use tokio::sync::mpsc::{self};
use tokio::sync::oneshot::{channel, Sender};

pub struct BlockWorld {
  regions: HashMap<RegionPosition, RegionHandle>,
  loaded_chunks: HashSet<super::ChunkPosition>,
  pub id: NamespacedKey,
  pub generator: Box<dyn ChunkGenerator>,
  pub loader: Box<dyn ChunkLoader>,
  pub entities: EntityList,
  pub spawn_position: Vec3d<f64>,
}

impl BlockWorld {
  pub fn new<G, L>(id: NamespacedKey, generator: G, loader: L) -> Self
  where
    G: Into<Box<dyn ChunkGenerator>>,
    L: Into<Box<dyn ChunkLoader>>,
  {
    Self {
      id,
      regions: HashMap::new(),
      loaded_chunks: HashSet::new(),
      generator: generator.into(),
      loader: loader.into(),
      entities: Default::default(),
      spawn_position: Vec3d::new(8.0, 16.0, 8.0),
    }
  }
  /// Returns a block at the given world position. If the chunk containing the
  /// block is not loaded, it is loaded/generated if `load = true`, otherwise
  /// `Block::Air` will be returned.
  pub async fn get_block_at_pos(&mut self, pos: Vec3d<i32>, load: bool) -> Block {
    if !(0..256).contains(pos.get_y_as_ref()) {
      // Outside of chunk building limit
      return Block::Air;
    }
    let region_position: RegionPosition = pos.into();
    let loaded = self.regions.get_mut(&region_position);
    if let Some(loaded) = loaded {
      let chunk_loaded = self
        .loaded_chunks
        .contains(&super::ChunkPosition::from(pos));
      if !chunk_loaded {
        if load {
          let chk_pos = super::ChunkPosition::from(pos);
          let chunk = if let Some(loaded_chk) = self.loader.load_chunk(chk_pos) {
            loaded_chk
          } else {
            self.generator.generate(chk_pos)
          };
          let block = chunk.get_block_at_pos(Vec3d::new(
            (pos.get_x() % 16) as u8,
            pos.get_y() as u8,
            (pos.get_z() % 16) as u8,
          ));
          loaded.set_chunk(chunk).await.unwrap();
          self.loaded_chunks.insert(chk_pos);
          block
        } else {
          Block::Air
        }
      } else {
        loaded
          .get_block(Vec3d::new(
            pos.get_x() % super::CHUNK_BLOCK_WIDTH as i32,
            pos.get_y(),
            pos.get_z() % super::CHUNK_BLOCK_WIDTH as i32,
          ))
          .await
          .expect("Unable to get block in region")
          .unwrap_or(Block::Air)
      }
    } else {
      Block::Air
    }
  }
}

#[async_trait]
impl Actor for BlockWorld {
  type Handle = WorldHandle;

  async fn handle_message(&mut self, message: <Self::Handle as ActorHandle>::Message) -> bool {
    match message {
      WorldMessage::GetBlockAtPos(pos, callback, load) => {
        let get_fut = self.get_block_at_pos(pos, load);
        match callback.send(get_fut.await) {
          Ok(()) => (),
          Err(_) => {
            eprintln!("[world.rs] Failed to send GetBlockAtPos result");
          }
        }
        true
      }
      WorldMessage::GetSpawnPosition(callback) => {
        match callback.send(self.spawn_position) {
          Ok(()) => (),
          Err(_) => {
            eprintln!("[world.rs] Failed to send GetSpawnPosition result");
          }
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
}

pub enum WorldMessage {
  /// A position and a callback
  GetBlockAtPos(Vec3d<i32>, Sender<Block>, bool),
  GetSpawnPosition(Sender<Vec3d<f64>>),
}

pub type WorldHandle = ActorHandleStruct<WorldMessage>;

impl WorldHandle {
  /// Gets the block at the specified position
  pub async fn get_block_at_pos(
    &mut self,
    pos: Vec3d<i32>,
    load_if_needed: bool,
  ) -> Result<Block, ()> {
    let (send, recv) = channel();
    match self
      .send_raw_message(ActorMessage::Other(WorldMessage::GetBlockAtPos(
        pos,
        send,
        load_if_needed,
      )))
      .await
    {
      Ok(_) => Ok(recv.await.unwrap()),
      Err(_) => Err(()),
    }
  }
  pub async fn get_spawn_position(&mut self) -> Result<Vec3d<f64>, ()> {
    let (send, recv) = channel();
    match self
      .send_raw_message(ActorMessage::Other(WorldMessage::GetSpawnPosition(send)))
      .await
    {
      Ok(_) => Ok(recv.await.unwrap()),
      Err(_) => Err(()),
    }
  }
}

impl fmt::Display for BlockWorld {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "BlockWorld '{}'", self.id)
  }
}
