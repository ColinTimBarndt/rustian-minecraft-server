use crate::helpers::Vec3d;
use crate::packet::{data::write, packet_ids::PLAY_CB_SPAWN_POSITION, PacketSerialOut};

/// # Spawn Position
/// [Documentation](https://wiki.vg/Protocol#Spawn_Position)
///
/// Sent by the server after login to specify the coordinates of the spawn point (the point at which
/// players spawn at, and which the compass points to). It can be sent at any time to update the
/// point compasses point at.
#[derive(Debug, Clone, Copy)]
pub struct SpawnPosition {
  pub position: Vec3d<i32>,
}

impl PacketSerialOut for SpawnPosition {
  const ID: u32 = PLAY_CB_SPAWN_POSITION;
  fn write(&self, buffer: &mut Vec<u8>) -> Result<(), String> {
    write::block_position(buffer, &self.position)?;
    Ok(())
  }
}
