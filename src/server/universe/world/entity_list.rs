use crate::server::universe::entity::player::EntityPlayer;
use std::collections::HashMap;

pub struct EntityList {
  pub player: HashMap<u32, EntityPlayer>,
}

impl Default for EntityList {
  fn default() -> Self {
    Self {
      player: HashMap::new(),
    }
  }
}
