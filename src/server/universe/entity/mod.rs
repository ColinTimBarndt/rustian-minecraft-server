use crate::actor_model::*;
use crate::server::registries::EntityType;
use async_trait::async_trait;
use std::any::Any;
use tokio::sync::mpsc::{error::SendError, Sender};

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

#[async_trait]
pub trait EntityActorHandle: ActorHandle {
  /// Returns the entity id for this entity actor
  fn get_id(&self) -> u32;
}

#[derive(Debug)]
pub struct EntityActorHandleStruct<M: Sized + Send + 'static, F: Sized + Send + Sync + 'static = ()>
{
  pub(super) sender: Sender<ActorMessage<M>>,
  pub(super) id: u32,
  pub(super) final_properties: std::sync::Arc<F>,
}

impl<M, F> Clone for EntityActorHandleStruct<M, F>
where
  M: Sized + Send + 'static,
  F: Sized + Send + Sync + 'static,
{
  fn clone(&self) -> Self {
    Self {
      sender: self.sender.clone(),
      id: self.id,
      final_properties: self.final_properties.clone(),
    }
  }
}

#[async_trait]
impl<M, F> ActorHandle for EntityActorHandleStruct<M, F>
where
  M: Sized + Send + 'static,
  F: Sized + Send + Sync + 'static,
{
  type Message = M;
  async fn send_raw_message(
    &mut self,
    message: ActorMessage<M>,
  ) -> Result<(), SendError<ActorMessage<M>>> {
    self.sender.send(message).await
  }
}

impl<M, F> EntityActorHandle for EntityActorHandleStruct<M, F>
where
  M: Sized + Send + 'static,
  F: Sized + Send + Sync + 'static,
{
  fn get_id(&self) -> u32 {
    self.id
  }
}

impl<M, F> EntityActorHandleStruct<M, F>
where
  M: Sized + Send + 'static,
  F: Sized + Send + Sync + 'static,
{
  pub fn new(id: u32, sender: Sender<ActorMessage<M>>, final_properties: F) -> Self {
    Self {
      id,
      sender,
      final_properties: std::sync::Arc::new(final_properties),
    }
  }
}
