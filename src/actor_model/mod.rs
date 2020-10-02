use async_trait::async_trait;
use tokio::sync::mpsc::channel;
use tokio::sync::mpsc::{error::SendError, Receiver, Sender};
use tokio::task::JoinHandle;

/// Structs that implement this trait are actors. They
/// run in a separate thread and can only communicate
/// using messages
#[async_trait]
pub trait Actor: Sized + std::fmt::Display + 'static {
  type Handle: ActorHandle;
  const BUFFER_SIZE: usize = 100;

  fn spawn_actor(mut self) -> (JoinHandle<Self>, Self::Handle)
  where
    Self: Send + 'static,
  {
    let (send, recv) = channel(Self::BUFFER_SIZE);
    let handle = self.create_handle(send);
    self.set_handle(handle.clone());
    let fut = tokio::spawn(async move { self.start_actor(recv).await });
    (fut, handle)
  }

  async fn start_actor(
    mut self,
    mut recv: Receiver<ActorMessage<<Self::Handle as ActorHandle>::Message>>,
  ) -> Self {
    loop {
      match recv.recv().await {
        None => {
          eprintln!("All handles for Actor 🧍 '{}' were dropped", self);
          break;
        }
        Some(ActorMessage::StopActor) => {
          break;
        }
        Some(ActorMessage::Other(msg)) => {
          if !self.handle_message(msg).await {
            break;
          }
        }
      }
    }
    self
  }

  /// Returns true if the actor 🧍 should continue running
  async fn handle_message(&mut self, message: <Self::Handle as ActorHandle>::Message) -> bool;

  fn create_handle(
    &self,
    _sender: Sender<ActorMessage<<Self::Handle as ActorHandle>::Message>>,
  ) -> Self::Handle;

  /// Stores the handle of this actor for future use
  fn set_handle(&mut self, handle: Self::Handle);

  /// Gets the stored handle. This function should panic
  /// if the actor has not been spawned and it should
  /// only be used inside the `handle_message` function.
  fn clone_handle(&self) -> Self::Handle;
}

/// A message an actor can receive
#[derive(Debug)]
pub enum ActorMessage<M: Sized + Send + 'static> {
  StopActor,
  Other(M),
}

#[derive(Debug)]
pub struct ActorHandleStruct<M: Sized + Send + 'static> {
  pub(super) sender: Sender<ActorMessage<M>>,
}

/// A handle for communicating with an actor
#[async_trait]
pub trait ActorHandle: Clone + Sized + Send + 'static {
  type Message: Sized + Send + 'static;

  async fn send_raw_message(
    &mut self,
    message: ActorMessage<Self::Message>,
  ) -> Result<(), SendError<ActorMessage<Self::Message>>>;

  async fn stop_actor(&mut self) -> Result<(), ()> {
    match self.send_raw_message(ActorMessage::StopActor).await {
      Ok(_) => Ok(()),
      Err(_) => Err(()),
    }
  }
}

impl<M: Sized + Send + 'static> Clone for ActorHandleStruct<M> {
  fn clone(&self) -> Self {
    Self {
      sender: self.sender.clone(),
    }
  }
}

#[async_trait]
impl<M: Sized + Send + 'static> ActorHandle for ActorHandleStruct<M> {
  type Message = M;
  async fn send_raw_message(
    &mut self,
    message: ActorMessage<M>,
  ) -> Result<(), SendError<ActorMessage<M>>> {
    self.sender.send(message).await
  }
}

impl<M: Sized + Send + 'static> From<Sender<ActorMessage<M>>> for ActorHandleStruct<M> {
  fn from(sender: Sender<ActorMessage<M>>) -> Self {
    Self { sender }
  }
}

/*
// Not supported yet
default<T: CreateHandle> impl Actor for T {
  fn spawn_actor(mut self) -> (JoinHandle<Self>, Self::Handle) {
    let (mut send, recv) = channel(Self::BUFFER_SIZE);
    let fut = tokio::spawn(async move { self.start_actor(recv).await });
    (fut, Self::Handle::from_sender(send))
  }
}
*/
