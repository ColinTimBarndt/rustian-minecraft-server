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
use tokio::sync::mpsc;
use tokio::sync::oneshot;
use tokio::sync::RwLock;
use tokio::task::JoinHandle;

pub static OVERWORLD_KEY: &'static str = "overworld";
pub static NETHER_KEY: &'static str = "nether";
pub static THE_END_KEY: &'static str = "the_end";
type SharedTagList<T> = RwLock<TagList<T>>;

pub struct Universe {
  pub tags: Arc<UniverseTags>,
  /// Stores all worlds in this universe as a world handle
  /// to the world actor
  worlds: HashMap<NamespacedKey, world::WorldHandle>,
  world_futures: HashMap<NamespacedKey, JoinHandle<world::BlockWorld>>,
  pub default_world: NamespacedKey,
  pub eid_gen: EntityIdGenerator,
  handle: Option<<Universe as Actor>::Handle>,
}

impl Universe {
  pub fn new(default_world: NamespacedKey) -> Self {
    Self {
      tags: Arc::new(Default::default()),
      eid_gen: EntityIdGenerator::new(),
      worlds: HashMap::new(),
      world_futures: HashMap::new(),
      default_world,
      handle: None,
    }
  }

  /// Adds a world to this universe. This function returns an error
  /// if a world with the same namespaced key already exists.
  fn add_world(&mut self, world: world::BlockWorld) -> Result<(), ()> {
    if self.worlds.contains_key(&world.id) {
      return Err(());
    }
    let id = world.id.clone();
    let (jh, handle) = world.spawn_actor();
    self.worlds.insert(id.clone(), handle);
    self.world_futures.insert(id, jh);
    Ok(())
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

#[derive(Debug)]
pub enum WorldCreationError {
  WorldNameAlreadyExists,
  MessagingError(ActorMessagingError),
}

impl From<ActorMessagingError> for WorldCreationError {
  fn from(e: ActorMessagingError) -> Self {
    Self::MessagingError(e)
  }
}

impl UniverseHandle {
  pub async fn create_world(
    &mut self,
    id: NamespacedKey,
    generator: Box<dyn world::chunk_generator::ChunkGenerator>,
    loader: Box<dyn world::chunk_loader::ChunkLoader>,
  ) -> Result<(), WorldCreationError> {
    let (send, recv) = oneshot::channel();
    self
      .send_raw_message(ActorMessage::Other(UniverseMessage::CreateWorld {
        key: id,
        generator: generator.into(),
        loader: loader.into(),
        callback: send,
      }))
      .await?;
    match recv.await.map_err(|e| ActorMessagingError::from(e))? {
      true => Ok(()),
      false => Err(WorldCreationError::WorldNameAlreadyExists),
    }
  }
  /// Gets a world by its namespaced key, which is unique for each universe
  pub async fn get_world(
    &mut self,
    id: NamespacedKey,
  ) -> ActorMessagingResult<Option<world::WorldHandle>> {
    let (send, recv) = oneshot::channel();
    self
      .send_raw_message(ActorMessage::Other(UniverseMessage::GetWorld(id, send)))
      .await?;
    Ok(recv.await?)
  }
  /// Returns a world where the given player would join
  pub async fn join_world(
    &mut self,
    player: uuid::Uuid,
  ) -> ActorMessagingResult<world::WorldHandle> {
    let (send, recv) = oneshot::channel();
    self
      .send_raw_message(ActorMessage::Other(UniverseMessage::JoinWorld(
        player, send,
      )))
      .await?;
    Ok(recv.await?)
  }
  pub async fn reserve_entity_id(&mut self) -> ActorMessagingResult<u32> {
    let (send, recv) = oneshot::channel();
    self
      .send_raw_message(ActorMessage::Other(UniverseMessage::ReserveEntityId(send)))
      .await?;
    Ok(recv.await?)
  }
  pub async fn free_entity_id(&mut self, id: u32) -> ActorMessagingResult {
    self
      .send_raw_message(ActorMessage::Other(UniverseMessage::FreeEntityId(id)))
      .await
  }
}

#[async_trait]
impl Actor for Universe {
  type Handle = UniverseHandle;

  async fn handle_message(&mut self, message: <Self::Handle as ActorHandle>::Message) -> bool {
    match message {
      UniverseMessage::CreateWorld {
        key,
        generator,
        loader,
        callback,
      } => {
        let _ = match self.add_world(world::BlockWorld::new(
          self.clone_handle(),
          key,
          generator,
          loader,
        )) {
          Ok(()) => callback.send(true),
          Err(()) => callback.send(false),
        };
        true
      }
      UniverseMessage::GetWorld(id, callback) => {
        let world = self.worlds.get(&id).map(|handle| handle.clone());
        let _ = callback.send(world);
        true
      }
      UniverseMessage::JoinWorld(_player, callback) => {
        let world = self.worlds.get(&self.default_world);
        if let Some(world) = world {
          let _ = callback.send(world.clone());
        } else {
          eprintln!("Default world '{}' does not exist", self.default_world);
          drop(callback);
        }
        true
      }
      UniverseMessage::ReserveEntityId(callback) => {
        let eid = self.eid_gen.reserve();
        match callback.send(eid) {
          Ok(()) => (),
          Err(_) => self.eid_gen.free(eid),
        }
        true
      }
      UniverseMessage::FreeEntityId(id) => {
        // Does not do anything if the id is not reserved
        self.eid_gen.free(id);
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

pub enum UniverseMessage {
  CreateWorld {
    key: NamespacedKey,
    generator: Box<dyn world::chunk_generator::ChunkGenerator>,
    loader: Box<dyn world::chunk_loader::ChunkLoader>,
    callback: oneshot::Sender<bool>,
  },
  GetWorld(NamespacedKey, oneshot::Sender<Option<world::WorldHandle>>),
  JoinWorld(uuid::Uuid, oneshot::Sender<world::WorldHandle>),
  ReserveEntityId(oneshot::Sender<u32>),
  FreeEntityId(u32),
}

impl fmt::Display for Universe {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Universe")
  }
}
