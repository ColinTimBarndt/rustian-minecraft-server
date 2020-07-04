use super::block::Block;
use super::chunk_generator::ChunkGenerator;
use super::chunk_loader::ChunkLoader;
use super::region::*;
use super::EntityList;
use crate::actor_model::*;
use crate::helpers::{NamespacedKey, Vec3d};
use async_trait::async_trait;
use std::collections::{HashMap, HashSet};
use std::fmt;
use tokio::sync::mpsc::error::SendError;
use tokio::sync::oneshot::{channel, Sender};

pub struct BlockWorld {
  regions: HashMap<RegionPosition, RegionHandle>,
  loaded_chunks: HashSet<super::ChunkPosition>,
  pub id: NamespacedKey,
  pub generator: Box<dyn ChunkGenerator>,
  pub loader: Box<dyn ChunkLoader>,
  pub entities: EntityList,
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
    }
  }
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
          // TODO: Load
        } else {
          return Block::Air;
        }
      }
      loaded
        .get_block(Vec3d::new(
          pos.get_x() / super::CHUNK_BLOCK_WIDTH as i32,
          pos.get_y(),
          pos.get_z() / super::CHUNK_BLOCK_WIDTH as i32,
        ))
        .await
        .expect("Unable to get block in region")
        .unwrap_or(Block::Air)
    } else {
      Block::Air
    }
  }
}

#[async_trait]
impl Actor for BlockWorld {
  type Message = WorldMessage;
  type Handle = WorldHandle;

  async fn handle_message(&mut self, message: WorldMessage) -> bool {
    match message {
      WorldMessage::GetBlockAtPos(pos, callback, load) => {
        let get_fut = self.get_block_at_pos(pos, load);
        match callback.send(get_fut.await) {
          Ok(()) => (),
          Err(_) => {
            eprintln!("Failed to send GetBlockAtPos result");
          }
        }
        true
      }
    }
  }
}

pub enum WorldMessage {
  /// A position and a callback
  GetBlockAtPos(Vec3d<i32>, Sender<Block>, bool),
}

pub type WorldHandle = ActorHandleStruct<WorldMessage>;

impl WorldHandle {
  /// Gets the block at the specified position
  pub async fn get_block_at_pos(
    &mut self,
    pos: Vec3d<i32>,
    load_if_needed: bool,
  ) -> Result<Block, SendError<ActorMessage<WorldMessage>>> {
    let (send, recv) = channel();
    self
      .send_raw_message(ActorMessage::Other(WorldMessage::GetBlockAtPos(
        pos,
        send,
        load_if_needed,
      )))
      .await?;
    Ok(recv.await.expect("Sender for GetBlockAtPos got dropped"))
  }
}

impl fmt::Display for BlockWorld {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "BlockWorld '{}'", self.id)
  }
}
