use crate::server::registries::EntityType;
use std::any::Any;

pub mod player;

pub trait Entity: Sized + 'static {
  const ENTITY_TYPE: EntityType;
  fn get_id(&self) -> u32;

  fn as_any(&self) -> &dyn Any {
    self
  }
}

pub trait EntityLiving: Entity {
  fn get_health(&self) -> f32;
  fn set_health(&mut self, health: f32);
  fn damage(&mut self, damage: f32) {
    self.set_health((self.get_health() - damage).max(0.0));
  }
}
