use crate::helpers::NibbleArray4096;
use crate::packet::{data::write, PacketSerialOut};
use crate::server::universe::world::{Chunk, ChunkPosition, LightSection};

/// # Update Light
/// [Documentation](https://wiki.vg/Protocol#Update_Light)
///
/// Updates light levels for a chunk.
#[derive(Clone, Copy)]
pub struct UpdateLight<'a> {
  pub chunk_position: ChunkPosition,
  // TODO: What is the purpose of this flag?
  pub trust_edges: bool,
  pub sky_light: &'a [LightSection; 18],
  pub emitted_light: &'a [LightSection; 18],
}

impl PacketSerialOut for UpdateLight<'_> {
  const ID: u32 = 0x25;
  fn write(&self, buffer: &mut Vec<u8>) -> Result<(), String> {
    self.clone().consume_write(buffer)
  }
  fn consume_write(self, buffer: &mut Vec<u8>) -> Result<(), String> {
    write::var_i32(buffer, self.chunk_position.x);
    write::var_i32(buffer, self.chunk_position.z);
    //write::bool(buffer, self.trust_edges);
    let sky_mask = create_masks(&self.sky_light);
    let emit_mask = create_masks(&self.emitted_light);
    write::var_u32(buffer, sky_mask.0);
    write::var_u32(buffer, emit_mask.0);
    write::var_u32(buffer, sky_mask.1);
    write::var_u32(buffer, emit_mask.1);
    for ldata in self.sky_light.iter() {
      if let LightSection::Some(data) = ldata {
        let raw: &[u8; 2048] = data.as_ref();
        write::var_usize(buffer, raw.len());
        buffer.extend_from_slice(raw);
      }
    }
    for ldata in self.emitted_light.iter() {
      if let LightSection::Some(data) = ldata {
        let raw: &[u8; 2048] = data.as_ref();
        write::var_usize(buffer, raw.len());
        buffer.extend_from_slice(raw);
      }
    }
    Ok(())
  }
}

impl<'a> UpdateLight<'a> {
  pub fn from_chunk(chunk: &'a Chunk, trust_edges: bool) -> Self {
    Self {
      chunk_position: chunk.position,
      trust_edges,
      sky_light: &chunk.sky_light,
      emitted_light: &chunk.emitted_light,
    }
  }
}

fn create_masks(data: &[LightSection; 18]) -> (u32, u32) {
  let mut include = 0;
  let mut empty = 0;
  for i in 0..18 {
    match data[i] {
      LightSection::None => (),
      LightSection::Zero => empty |= 1 << i,
      LightSection::Some(_) => include |= 1 << i,
    }
  }
  (include, empty)
}
