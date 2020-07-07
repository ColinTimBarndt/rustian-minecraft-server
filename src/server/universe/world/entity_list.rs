use crate::server::registries::EntityType;
use crate::server::universe::entity::player::online_controller::ControllerHandle as OnlinePlayerHandle;
use crate::server::universe::entity::EntityActorHandle;
use std::collections::HashMap;

pub struct EntityList {
  entity_types: HashMap<u32, EntityType>,
  player: HashMap<u32, OnlinePlayerHandle>,
}

impl Default for EntityList {
  fn default() -> Self {
    Self {
      entity_types: HashMap::new(),
      player: HashMap::new(),
    }
  }
}

impl EntityList {
  pub fn new() -> Self {
    Default::default()
  }
  pub fn get_type_of_entity(&self, id: u32) -> Option<EntityType> {
    match self.entity_types.get(&id) {
      Some(&t) => Some(t),
      None => None,
    }
  }
}

pub trait EntityListEntity<Handle: EntityActorHandle> {
  fn get(&self, id: u32) -> Option<&Handle>;
  fn get_mut(&mut self, id: u32) -> Option<&mut Handle>;
  fn set(&mut self, handle: Handle);
}

impl EntityListEntity<OnlinePlayerHandle> for EntityList {
  fn get(&self, id: u32) -> Option<&OnlinePlayerHandle> {
    self.player.get(&id)
  }
  fn get_mut(&mut self, id: u32) -> Option<&mut OnlinePlayerHandle> {
    self.player.get_mut(&id)
  }
  fn set(&mut self, handle: OnlinePlayerHandle) {
    self.player.insert(handle.get_id(), handle);
  }
}
