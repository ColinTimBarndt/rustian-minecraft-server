use crate::packet::{data::write, PacketSerialOut};
use crate::server::universe::world::ChunkPosition;

#[derive(Debug, Clone, Copy)]
pub struct UpdateViewPosition {
  pub position: ChunkPosition,
}

impl PacketSerialOut for UpdateViewPosition {
  const ID: u32 = 0x41;
  fn write(&self, buffer: &mut Vec<u8>) -> Result<(), String> {
    write::var_i32(buffer, self.position.get_x());
    write::var_i32(buffer, self.position.get_z());
    Ok(())
  }
}
