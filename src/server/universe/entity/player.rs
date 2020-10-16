use super::*;
use crate::helpers::{chat_components, EulerAngle, Vec3d};
use crate::server::registries::EntityType;

pub mod game_profile;
mod settings;
pub use settings::*;

/// Represents the player entity
pub struct EntityPlayer {
  pub id: u32,
  pub custom_name: Option<Vec<chat_components::ChatComponent>>,
  pub position: Vec3d<f64>,
  pub head_rotation: EulerAngle,
  pub health: f32,
  pub food: f32,
  pub saturation: f32,
  pub selected_hotbar_slot: u8,
  pub on_ground: bool,
  pub settings: PlayerSettings,
  pub profile: game_profile::GameProfile,
}

impl EntityPlayer {
  pub fn new(id: u32, profile: game_profile::GameProfile) -> Self {
    Self {
      id,
      profile,
      ..Default::default()
    }
  }
}

impl Default for EntityPlayer {
  fn default() -> Self {
    Self {
      id: 0,
      custom_name: None,
      position: Default::default(),
      head_rotation: Default::default(),
      health: 20.0,
      food: 20.0,
      saturation: 20.0,
      selected_hotbar_slot: 0,
      on_ground: false,
      settings: Default::default(),
      profile: game_profile::GameProfile::new_offline("Steve"),
    }
  }
}

impl Entity for EntityPlayer {
  const ENTITY_TYPE: EntityType = EntityType::Player;
  fn get_id(&self) -> u32 {
    self.id
  }
}

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
    pub entity: EntityPlayer,
    handle: Option<<Controller as Actor>::Handle>,
  }

  /// Other messages this Actor can receive
  pub enum ControllerMessage {
    /// Client updated settings
    UpdateSettings(PlayerSettings),
    /// Client (`update_client: false`) or
    /// server (`update_client: true`) changes hotbar slot
    SetSelectedHotbarSlot { slot: u8, update_client: bool },
    /// Client moved
    PlayerMoved {
      position: Option<Vec3d<f64>>,
      rotation: Option<EulerAngle>,
      on_ground: bool,
    },
  }

  impl ControllerHandle {
    pub fn get_name(&self) -> String {
      self.final_properties.name.clone()
    }
    pub fn get_uuid(&self) -> uuid::Uuid {
      self.final_properties.uuid.clone()
    }
    pub async fn update_settings(&mut self, settings: PlayerSettings) -> Result<(), ()> {
      self
        .send_raw_message(ActorMessage::Other(ControllerMessage::UpdateSettings(
          settings,
        )))
        .await
        .map_err(|_| ())
    }
    pub async fn set_selected_hotbar_slot(
      &mut self,
      slot: u8,
      update_client: bool,
    ) -> Result<(), ()> {
      assert!(slot <= 9, "Invalid slot id: {}", slot);
      self
        .send_raw_message(ActorMessage::Other(
          ControllerMessage::SetSelectedHotbarSlot {
            slot,
            update_client,
          },
        ))
        .await
        .map_err(|_| ())
    }
    pub async fn player_moved(
      &mut self,
      position: Option<Vec3d<f64>>,
      rotation: Option<EulerAngle>,
      on_ground: bool,
    ) -> Result<(), ()> {
      self
        .send_raw_message(ActorMessage::Other(ControllerMessage::PlayerMoved {
          position,
          rotation,
          on_ground,
        }))
        .await
        .map_err(|_| ())
    }
  }

  /// Handle for communicating with this actor
  pub type ControllerHandle =
    super::EntityActorHandleStruct<ControllerMessage, SharedControllerProperties>;

  impl Controller {
    pub fn new(con: PlayerConnectionPacketHandle, entity: EntityPlayer) -> Self {
      Self {
        connection: con,
        entity,
        handle: None,
      }
    }

    pub async fn set_selected_hotbar_slot(&mut self, slot: u8) -> Result<(), String> {
      let packet = crate::packet::play::send::HeldItemChange { hotbar_slot: slot };
      let con = &mut self.connection;
      con.send_packet(packet).await?;
      self.entity.selected_hotbar_slot = slot;
      Ok(())
    }
  }

  pub struct SharedControllerProperties {
    name: String,
    uuid: uuid::Uuid,
  }

  #[async_trait]
  impl Actor for Controller {
    type Handle = ControllerHandle;

    async fn handle_message(&mut self, message: <Self::Handle as ActorHandle>::Message) -> bool {
      match message {
        ControllerMessage::UpdateSettings(settings_new) => {
          self.entity.settings = settings_new;
          // TODO: Send chunks etc
          true
        }
        ControllerMessage::SetSelectedHotbarSlot {
          update_client: false,
          slot,
        } => {
          self.entity.selected_hotbar_slot = slot;
          true
        }
        ControllerMessage::SetSelectedHotbarSlot {
          update_client: true,
          slot,
        } => match self.set_selected_hotbar_slot(slot).await {
          Ok(()) => true,
          Err(e) => {
            eprintln!("Communication with Sender Actor failed: {}", e);
            false
          }
        },
        ControllerMessage::PlayerMoved {
          position,
          rotation,
          on_ground,
        } => {
          // TODO: movement validation
          self.entity.on_ground = on_ground;
          if let Some(position) = position {
            self.entity.position = position;
          }
          if let Some(rotation) = rotation {
            self.entity.head_rotation = rotation;
          }
          true
        }
      }
    }

    fn create_handle(
      &self,
      sender: Sender<ActorMessage<<Self::Handle as ActorHandle>::Message>>,
    ) -> Self::Handle {
      ControllerHandle::new(
        self.entity.id,
        sender,
        SharedControllerProperties {
          name: self.entity.profile.name.clone(),
          uuid: self.entity.profile.uuid.clone(),
        },
      )
    }

    fn set_handle(&mut self, handle: Self::Handle) {
      self.handle = Some(handle);
    }

    fn clone_handle(&self) -> Self::Handle {
      self.handle.as_ref().unwrap().clone()
    }
  }

  impl fmt::Display for Controller {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(
        f,
        "Online Player Controller ({} / {})",
        self.entity.profile.name, self.entity.profile.uuid
      )
    }
  }
}
