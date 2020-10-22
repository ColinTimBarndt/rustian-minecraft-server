use crate::packet::{data::write, packet_ids::PLAY_CB_UPDATE_VIEW_POSITION, PacketSerialOut};
use crate::server::universe::world::ChunkPosition;

#[derive(Debug, Clone, Copy)]
pub struct UpdateViewPosition {
  pub position: ChunkPosition,
}

impl PacketSerialOut for UpdateViewPosition {
  const ID: u32 = PLAY_CB_UPDATE_VIEW_POSITION;
  fn write(&self, buffer: &mut Vec<u8>) -> Result<(), String> {
    write::var_i32(buffer, self.position.x);
    write::var_i32(buffer, self.position.z);
    Ok(())
  }
}
