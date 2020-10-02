use crate::actor_model::Actor;
use crate::server::registries::EntityType;
use crate::server::universe::entity::player::online_controller::Controller as OnlinePlayer;
use crate::server::universe::entity::{Entity, EntityActorHandle};
use std::collections::HashMap;
use tokio::task::JoinHandle;

#[derive(Default)]
pub struct EntityList {
  entity_types: HashMap<u32, EntityType>,
  player_list: HashMap<u32, (JoinHandle<OnlinePlayer>, <OnlinePlayer as Actor>::Handle)>,
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

pub trait EntityListEntry<T: Actor>
where
  <T as Actor>::Handle: EntityActorHandle,
{
  fn get(&self, id: u32) -> Option<&(JoinHandle<T>, T::Handle)>;
  fn get_mut(&mut self, id: u32) -> Option<&mut (JoinHandle<T>, T::Handle)>;
  fn insert(&mut self, handle: (JoinHandle<T>, T::Handle));
  fn remove(&mut self, id: u32) -> Option<(JoinHandle<T>, T::Handle)>;
}

macro_rules! register_entity {
  ($var:ident = $ety:ident $ty:ty) => {
    impl EntityListEntry<$ty> for EntityList {
      fn get(&self, id: u32) -> Option<&(JoinHandle<$ty>, <$ty as Actor>::Handle)> {
        self.$var.get(&id)
      }
      fn get_mut(&mut self, id: u32) -> Option<&mut (JoinHandle<$ty>, <$ty as Actor>::Handle)> {
        self.$var.get_mut(&id)
      }
      fn insert(&mut self, handle: (JoinHandle<$ty>, <$ty as Actor>::Handle)) {
        let id = handle.1.get_id();
        self.entity_types.insert(id, EntityType::$ety);
        self.$var.insert(id, handle);
      }
      fn remove(&mut self, id: u32) -> Option<(JoinHandle<$ty>, <$ty as Actor>::Handle)> {
        self.$var.remove(&id)
      }
    }
  };
}

register_entity!(player_list = Player OnlinePlayer);
