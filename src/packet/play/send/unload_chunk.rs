use crate::packet::{data::write, PacketSerialOut};
use crate::server::universe::world::ChunkPosition;

/// [Documentation](https://wiki.vg/Protocol#Unload_Chunk)
#[derive(Clone, Debug)]
pub struct UnloadChunk {
  pub chunk_position: ChunkPosition,
}

impl PacketSerialOut for UnloadChunk {
  const ID: u32 = 0x1E;
  fn write(&self, buffer: &mut Vec<u8>) -> Result<(), String> {
    write::i32(buffer, self.chunk_position.get_x());
    write::i32(buffer, self.chunk_position.get_z());
    Ok(())
  }
}
