use crate::helpers::{BitArray, NibbleArray4096};
use crate::packet::{data::write, PacketSerialOut};
use crate::server::universe::world::ChunkSection;
use crate::server::universe::world::{block, Chunk, ChunkPosition};
use std::mem;
extern crate nbt;
use nbt::Value;

/// # Chunk Data
/// [Documentation](https://wiki.vg/Protocol#Chunk_Data)
///
/// _Main article: [Chunk Format](https://wiki.vg/Chunk_Format)_
///
/// _See also: [#Unload Chunk](https://wiki.vg/Protocol#Unload_Chunk), [`crate::packet::play::send::UnloadChunk`]_
#[derive(Clone)]
pub struct ChunkData {
  pub chunk_position: ChunkPosition,
  pub heightmaps: ChunkDataHeightmaps,
  pub biomes: Option<[u32; 1024]>,
  pub sections: [Option<ChunkSectionData>; 16],
  pub block_entities: Vec<std::collections::HashMap<String, nbt::Value>>,
}

/// Each map has 9 bits ber entry
#[derive(Clone, Debug)]
pub struct ChunkDataHeightmaps {
  pub motion_blocking: BitArray,
  pub world_surface: BitArray,
}

#[derive(Clone, Debug)]
pub struct ChunkSectionData {
  pub solid_blocks: u16,
  pub palette: Option<Vec<u16>>,
  pub blocks: BitArray,
}

impl PacketSerialOut for ChunkData {
  const ID: u32 = 0x22;
  fn write(&self, buffer: &mut Vec<u8>) -> Result<(), String> {
    self.clone().consume_write(buffer)
  }
  fn consume_write(mut self, buffer: &mut Vec<u8>) -> Result<(), String> {
    // see https://wiki.vg/Chunk_Format#Packet_structure
    write::i32(buffer, self.chunk_position.get_x()); // Chunk X
    write::i32(buffer, self.chunk_position.get_z()); // Chunk Y
    write::bool(buffer, self.biomes.is_some()); // Full chunk
    let mut sections_total = 0;
    {
      let mut mask = 0u8;
      let mut i = 0;
      for chunk in self.sections.iter() {
        if chunk.is_some() {
          mask |= 1 << i;
          sections_total += 1;
        }
        i += 1;
      }
      write::var_u8(buffer, mask); // Primary Bit Mask
    }
    self.heightmaps.write(buffer); // Heightmaps
    if let Some(biomes) = self.biomes {
      for biome in biomes.iter() {
        write::u32(buffer, *biome); // Biomes
      }
    }
    // Initlializing the array with a very rough approximation of the required heap
    let mut data = Vec::with_capacity(sections_total * (2 + 1 + 2048));
    for i in 0..self.sections.len() {
      // I take the chunk section out of the array, owning it
      // Then I need to replace it with another value so I don't run into any severe runtime errors
      let section: Option<ChunkSectionData> = unsafe { mem::replace(&mut self.sections[i], None) };
      if let Some(section) = section {
        section.write(&mut data);
      }
    }
    write::var_usize(buffer, data.len()); // Size
    buffer.append(&mut data); // Data / Array of Chunk Section
    drop(data);
    write::var_usize(buffer, self.block_entities.len()); // Number of block entities
    for be in self.block_entities {
      nbt::to_writer(buffer, &nbt::Value::Compound(be), None); // Block entities
    }
    // `self` is dropped here
    Ok(())
  }
}

impl ChunkDataHeightmaps {
  pub fn write(self, buffer: &mut Vec<u8>) {
    // This code is safe, I just need to convert a Vec<u64> into a Vec<i64> with the same binary content
    let motion_blocking: Vec<i64> =
      unsafe { mem::transmute::<Vec<u64>, _>(self.motion_blocking.into()) };
    let world_surface: Vec<i64> =
      unsafe { mem::transmute::<Vec<u64>, _>(self.world_surface.into()) };
    let mut root = std::collections::HashMap::with_capacity(2);
    root.insert(
      "MOTION_BLOCKING".into(),
      nbt::Value::LongArray(motion_blocking),
    );
    root.insert("WORLD_SURFACE".into(), nbt::Value::LongArray(world_surface));
    let root = nbt::Value::Compound(root);
    nbt::to_writer(buffer, &root, None).expect("Failed to serialize NBT");
  }
}

impl ChunkSectionData {
  pub fn from_section(sec: &ChunkSection) -> Self {
    Self {
      solid_blocks: sec.get_solid_block_count(),
      palette: sec.get_palette().clone(),
      blocks: sec.get_raw_block_data().clone(),
    }
  }
  pub fn write(self, buffer: &mut Vec<u8>) {
    write::u16(buffer, self.solid_blocks);
    write::u8(buffer, self.blocks.bits_per_item());
    if let Some(palette) = self.palette {
      // Local palette
      write::var_usize(buffer, palette.len());
      for pal in palette {
        write::var_u16(buffer, pal);
      }
    } else {
      // Global palette
      write::var_u32(buffer, block::blocks::USED_PALETTE_BITS as u32);
    }
    let block_data: Vec<u64> = self.blocks.into();
    write::var_usize(buffer, block_data.len());
    for long in block_data {
      write::u64(buffer, long);
    }
  }
}

impl Default for ChunkDataHeightmaps {
  fn default() -> Self {
    Self {
      motion_blocking: BitArray::new(9, 256),
      world_surface: BitArray::new(9, 256),
    }
  }
}
