use async_trait::async_trait;
use tokio::sync::{mpsc, oneshot};
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
    let (send, recv) = mpsc::channel(Self::BUFFER_SIZE);
    let handle = self.create_handle(send);
    self.set_handle(handle.clone());
    let fut = tokio::spawn(async move { self.start_actor(recv).await });
    (fut, handle)
  }

  async fn start_actor(
    mut self,
    mut recv: mpsc::Receiver<ActorMessage<<Self::Handle as ActorHandle>::Message>>,
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
    _sender: mpsc::Sender<ActorMessage<<Self::Handle as ActorHandle>::Message>>,
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
  pub(super) sender: mpsc::Sender<ActorMessage<M>>,
}

/// A handle for communicating with an actor
#[async_trait]
pub trait ActorHandle: Clone + Sized + Send + 'static {
  type Message: Sized + Send + 'static;

  async fn send_raw_message(
    &mut self,
    message: ActorMessage<Self::Message>,
  ) -> ActorMessagingResult;

  async fn stop_actor(&mut self) -> ActorMessagingResult {
    self.send_raw_message(ActorMessage::StopActor).await
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
  async fn send_raw_message(&mut self, message: ActorMessage<M>) -> ActorMessagingResult {
    self
      .sender
      .send(message)
      .await
      .map_err(|_| ActorMessagingError::new("Failed to send raw actor message"))
  }
}

impl<M: Sized + Send + 'static> From<mpsc::Sender<ActorMessage<M>>> for ActorHandleStruct<M> {
  fn from(sender: mpsc::Sender<ActorMessage<M>>) -> Self {
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

#[derive(Debug, Clone, Copy)]
pub struct ActorMessagingError {
  message: &'static str,
}

pub type ActorMessagingResult<T = ()> = Result<T, ActorMessagingError>;

impl ActorMessagingError {
  pub const fn new(msg: &'static str) -> Self {
    Self { message: msg }
  }
}

impl std::fmt::Display for ActorMessagingError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
    write!(
      f,
      "Actor Messaging Error: \"{}\" (The other actor probably got dropped)",
      self.message
    )
  }
}

impl<T> From<mpsc::error::SendError<T>> for ActorMessagingError {
  fn from(_: mpsc::error::SendError<T>) -> Self {
    Self {
      message: "Failed to send message over MPSC channel",
    }
  }
}

impl From<oneshot::error::RecvError> for ActorMessagingError {
  fn from(_: oneshot::error::RecvError) -> Self {
    Self {
      message: "Failed to receive callback (The sender got dropped)",
    }
  }
}
