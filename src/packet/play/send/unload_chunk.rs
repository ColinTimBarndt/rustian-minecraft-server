use crate::packet::{data::write, packet_ids::PLAY_CB_UNLOAD_CHUNK, PacketSerialOut};
use crate::server::universe::world::ChunkPosition;

/// # Unload Chunk
/// [Documentation](https://wiki.vg/Protocol#Unload_Chunk)
///
/// Tells the client to unload a chunk column.
#[derive(Clone, Debug)]
pub struct UnloadChunk {
  pub chunk_position: ChunkPosition,
}

impl PacketSerialOut for UnloadChunk {
  const ID: u32 = PLAY_CB_UNLOAD_CHUNK;
  fn write(&self, buffer: &mut Vec<u8>) -> Result<(), String> {
    write::i32(buffer, self.chunk_position.x);
    write::i32(buffer, self.chunk_position.z);
    Ok(())
  }
}
