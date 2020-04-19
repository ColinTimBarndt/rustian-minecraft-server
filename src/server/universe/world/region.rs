use super::Chunk;
use crate::helpers::Vec3d;
use std::fmt;
use std::hash::{Hash, Hasher};
use tokio::sync::mpsc::{channel, error::SendError, Receiver, Sender};

pub const REGION_CHUNK_WIDTH: u32 = 2;
const COORDINATE_SCALE: i32 = (REGION_CHUNK_WIDTH * super::CHUNK_BLOCK_WIDTH) as i32;

pub struct Region {
  position: RegionPosition,
  chunks: [Chunk; (REGION_CHUNK_WIDTH * REGION_CHUNK_WIDTH) as usize],
}

impl Region {
  pub fn create_thread(mut self) -> RegionHandle {
    let (send, mut recv): (Sender<_>, Receiver<_>) = channel(4096);
    tokio::spawn(async move {
      use std::time::Duration;
      use tokio::sync::mpsc::error::{TryRecvError, TrySendError};
      let mut tick_interval = {
        let (mut s, r) = channel(128);
        tokio::spawn(async move {
          let mut intv = tokio::time::interval(Duration::from_secs_f32(1f32 / 20f32));
          loop {
            intv.tick().await;
            match s.try_send(()) {
              Ok(()) => (),
              Err(TrySendError::Full(_)) => {
                continue;
              }
              Err(TrySendError::Closed(_)) => {
                return;
              }
            }
          }
        });
        r
      };
      // This loop handles all incoming messages before processing the next tick
      loop {
        // Try to read a message
        match recv.try_recv() {
          Ok(msg) => match msg {
            // Process the message
            RegionHandleMessage::StopListening => {
              return;
            }
            #[allow(unreachable_patterns)]
            _ => {
              // Process the following messages or the next tick
              continue;
            }
          },
          Err(TryRecvError::Empty) => {
            // If no more messages are available, process the next tick if necessary
            // Ticks only fire every 1/20 seconds or more (depending on delay caused by messages)
            // If multiple ticks are missed, they stacked on the tick channel
            match tick_interval.try_recv() {
              Ok(()) => {
                // Do a tick
                self.tick();
              }
              Err(TryRecvError::Empty) => {
                continue;
              }
              Err(TryRecvError::Closed) => {
                return;
              }
            }
            continue;
          }
          Err(TryRecvError::Closed) => {
            return;
          }
        }
      }
    });

    RegionHandle::new(send)
  }
  pub fn tick(&mut self) {
    //
  }
}

#[derive(Debug)]
pub struct RegionPosition {
  x: i32,
  z: i32,
}
impl RegionPosition {
  pub fn new(x: i32, z: i32) -> Self {
    Self { x, z }
  }
  pub fn get_x(&self) -> i32 {
    self.x
  }
  pub fn get_z(&self) -> i32 {
    self.z
  }
  pub fn get_offset(&self) -> Vec3d<i32> {
    Vec3d::new(self.x * COORDINATE_SCALE, 0, self.z * COORDINATE_SCALE)
  }
}

impl Hash for RegionPosition {
  fn hash<H: Hasher>(&self, state: &mut H) {
    state.write_i32(self.x);
    state.write_i32(self.z);
  }
}

impl fmt::Display for RegionPosition {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "({}, {})", self.x, self.z)
  }
}

#[derive(Clone, Debug)]
pub struct RegionHandle {
  channel: Sender<RegionHandleMessage>,
}

impl RegionHandle {
  fn new(channel: Sender<RegionHandleMessage>) -> Self {
    Self { channel }
  }
  pub async fn stop_listening(&mut self) -> Result<(), SendError<RegionHandleMessage>> {
    self
      .channel
      .send(RegionHandleMessage::StopListening)
      .await?;
    Ok(())
  }
}

#[derive(Debug)]
pub enum RegionHandleMessage {
  StopListening,
}
