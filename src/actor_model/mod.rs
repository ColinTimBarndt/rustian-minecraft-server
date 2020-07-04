use async_trait::async_trait;
use tokio::sync::mpsc::channel;
use tokio::sync::mpsc::{error::SendError, Receiver, Sender};
use tokio::task::JoinHandle;

/// Structs that implement this trait are actors. They
/// run in a separate thread and can only communicate
/// using messages
#[async_trait]
pub trait Actor: Sized + std::fmt::Display + 'static {
  type Message: Sized + Send + 'static;
  type Handle: ActorHandle<Self::Message>;
  const BUFFER_SIZE: usize = 100;
  fn spawn_actor(mut self) -> (JoinHandle<Self>, Self::Handle)
  where
    Self: Send + 'static,
  {
    let (mut send, recv) = channel(Self::BUFFER_SIZE);
    let fut = tokio::spawn(async move { self.start_actor(recv).await });
    (fut, Self::Handle::from_sender(send))
  }
  async fn start_actor(mut self, mut recv: Receiver<ActorMessage<Self::Message>>) -> Self {
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
  async fn handle_message(&mut self, message: Self::Message) -> bool;
}

/// A message an actor can receive
#[derive(Debug)]
pub enum ActorMessage<M: Sized + Send + 'static> {
  StopActor,
  Other(M),
}

#[derive(Clone, Debug)]
pub struct ActorHandleStruct<M: Sized + Send + 'static> {
  pub(super) sender: Sender<ActorMessage<M>>,
}

/// A handle for communicating with an actor
#[async_trait]
pub trait ActorHandle<M: Sized + Send + 'static>: Sized + Send + 'static {
  fn from_sender(sender: Sender<ActorMessage<M>>) -> Self;
  async fn send_raw_message(
    &mut self,
    message: ActorMessage<M>,
  ) -> Result<(), SendError<ActorMessage<M>>>;
  async fn stop_actor(&mut self) -> Result<(), SendError<ActorMessage<M>>> {
    self.send_raw_message(ActorMessage::StopActor).await
  }
}

#[async_trait]
impl<M: Sized + Send + 'static> ActorHandle<M> for ActorHandleStruct<M> {
  fn from_sender(sender: Sender<ActorMessage<M>>) -> Self {
    Self { sender }
  }
  async fn send_raw_message(
    &mut self,
    message: ActorMessage<M>,
  ) -> Result<(), SendError<ActorMessage<M>>> {
    self.sender.send(message).await
  }
}
