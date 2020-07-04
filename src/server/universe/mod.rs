mod gamemode;
pub mod world;
pub use gamemode::*;
mod player;
pub use player::*;
pub mod entity;
mod entity_id_generator;
pub use entity_id_generator::*;

pub mod commands;
pub mod crafting;
pub mod item;
pub mod tags;

use crate::actor_model::*;
use crate::helpers::NamespacedKey;
use crate::server::registries;
use async_trait::async_trait;
use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;
use tags::TagList;
use tokio::sync::mpsc::{channel, error::SendError, Receiver, Sender};
use tokio::sync::RwLock;

pub static OVERWORLD_KEY: &'static str = "overworld";
pub static NETHER_KEY: &'static str = "nether";
pub static THE_END_KEY: &'static str = "the_end";
type SharedTagList<T> = RwLock<TagList<T>>;

pub struct Universe {
  pub tags: Arc<UniverseTags>,
  /// Stores all worlds in this universe as a world handle
  /// to the world actor
  worlds: HashMap<NamespacedKey, world::WorldHandle>,
  pub eid_gen: EntityIdGenerator,
}

impl Universe {
  pub fn new() -> Self {
    Self {
      tags: Arc::new(Default::default()),
      eid_gen: EntityIdGenerator::new(),
      worlds: HashMap::new(),
    }
  }
}

#[derive(Debug)]
pub struct UniverseTags {
  pub block_tags: SharedTagList<registries::Block>,
  pub item_tags: SharedTagList<registries::Item>,
  pub fluid_tags: SharedTagList<registries::Fluid>,
  pub entity_tags: SharedTagList<registries::EntityType>,
}

impl Default for UniverseTags {
  fn default() -> Self {
    Self {
      block_tags: RwLock::from(TagList::new()),
      item_tags: RwLock::from(TagList::new()),
      fluid_tags: RwLock::from(TagList::new()),
      entity_tags: RwLock::from(TagList::new()),
    }
  }
}

pub type UniverseHandle = ActorHandleStruct<UniverseMessage>;

#[async_trait]
impl Actor for Universe {
  type Message = UniverseMessage;
  type Handle = UniverseHandle;

  async fn handle_message(&mut self, _message: UniverseMessage) -> bool {
    true
  }
}

pub enum UniverseMessage {
  CreateWorld {
    key: NamespacedKey,
    loader: Box<dyn world::chunk_loader::ChunkLoader>,
    generator: Box<dyn world::chunk_generator::ChunkGenerator>,
  },
}

impl fmt::Display for Universe {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Universe")
  }
}
