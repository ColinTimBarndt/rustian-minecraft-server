use super::*;
use crate::helpers::Vec3d;
use crate::server::registries::EntityType;
use async_trait::async_trait;

/// Represents the player entity
pub struct EntityPlayer {
  id: u32,
  name: String,
  position: Vec3d<f64>,
  yaw: f32,
  pitch: f32,
  health: f32,
  food: f32,
  saturation: f32,
}

impl EntityPlayer {
  pub fn new(id: u32, name: String) -> Self {
    Self {
      id,
      name,
      position: Vec3d::new(0.0, 0.0, 0.0),
      yaw: 0.0,
      pitch: 0.0,
      health: 20.0,
      food: 20.0,
      saturation: 20.0,
    }
  }
}

#[async_trait]
impl Entity for EntityPlayer {
  const ENTITY_TYPE: EntityType = EntityType::Player;
  fn get_id(&self) -> u32 {
    self.id
  }
}

#[async_trait]
impl EntityLiving for EntityPlayer {
  fn get_health(&self) -> f32 {
    self.health
  }
  fn set_health(&mut self, health: f32) {
    self.health = health;
  }
}

pub mod online_controller {
  use super::*;
  use crate::packet::PlayerConnectionPacketHandle;
  use async_trait::async_trait;
  use std::fmt;

  /// Actor that handles player behavior
  pub struct Controller {
    pub connection: PlayerConnectionPacketHandle,
    pub player: EntityPlayer,
  }

  /// Other messages this Actor can receive
  pub enum ControllerMessage {
    // TODO
  }

  /// Handle for communicating with this actor
  pub type ControllerHandle = super::EntityActorHandleStruct<ControllerMessage>;

  #[async_trait]
  impl Actor for Controller {
    type Handle = ControllerHandle;

    async fn handle_message(&mut self, _message: <Self::Handle as ActorHandle>::Message) -> bool {
      true
    }

    fn create_handle(
      &self,
      sender: Sender<ActorMessage<<Self::Handle as ActorHandle>::Message>>,
    ) -> Self::Handle {
      ControllerHandle::new(self.player.id, sender)
    }
  }

  impl fmt::Display for Controller {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "Player Controller ({})", self.player.name)
    }
  }
}
