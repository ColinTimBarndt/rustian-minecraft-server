use crate::helpers::{BitArray, NibbleArray4096, Vec3d};
use std::collections::HashMap;

pub mod block;
mod dimension;
pub use block::Block;
pub use dimension::*;
mod level_type;
pub use level_type::*;
pub mod chunk_generator;
pub mod chunk_loader;

use chunk_generator::ChunkGenerator;
use chunk_loader::ChunkLoader;

pub struct BlockWorld<Generator: ChunkGenerator, Loader: ChunkLoader> {
    chunks: HashMap<u64, Chunk>,
    generator: Generator,
    loader: Loader,
}

impl<Generator, Loader> BlockWorld<Generator, Loader>
where
    Generator: ChunkGenerator,
    Loader: ChunkLoader,
{
    pub fn new(generator: Generator, loader: Loader) -> Self {
        Self {
            chunks: HashMap::new(),
            generator,
            loader,
        }
    }
    pub fn get_block_at_pos(&self, pos: Vec3d<i32>) -> Block {
        let chunk_position: ChunkPosition = pos.clone().into();
        let loaded = self.chunks.get(&chunk_position.get_u64());
        if let Some(loaded) = loaded {
            use crate::helpers::PositiveRem;
            let offset = pos.positive_rem(16);
            loaded.get_block_at_pos(Vec3d::new(
                offset.get_x() as u8,
                offset.get_y() as u8,
                offset.get_z() as u8,
            ))
        } else {
            Block::Air
        }
    }
}

// Base: https://github.com/feather-rs/feather/blob/develop/core/src/world/chunk.rs
// Newer: https://github.com/feather-rs/feather/blob/develop/core/src/chunk.rs

pub struct Chunk {
    pub sections: [Option<ChunkSection>; 16],
    position: ChunkPosition,
}

impl std::fmt::Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "Chunk({}, {})",
            self.position.get_x(),
            self.position.get_z()
        )
    }
}

impl Chunk {
    pub fn new_empty(pos: ChunkPosition) -> Self {
        Self {
            position: pos,
            ..Default::default()
        }
    }
    pub fn copy(&self, to: ChunkPosition) -> Self {
        Self {
            sections: self.sections.clone(),
            position: to,
        }
    }
    pub fn get_block_at_pos(&self, offset: Vec3d<u8>) -> Block {
        let y_index = (offset.get_y_as_ref() >> 4) as usize;
        let section = &self.sections[y_index];
        match section {
            Some(section) => {
                section.get_block_at_pos(offset.get_x(), offset.get_y(), offset.get_z())
            }
            None => Block::Air,
        }
    }
    pub fn set_block_at_pos(&mut self, offset: Vec3d<u8>, block: Block) {
        let y_index = (offset.get_y_as_ref() >> 4) as usize;
        let section = &mut self.sections[y_index];
        if let Some(section) = section.as_mut() {
            section.set_block_at_pos(offset.get_x(), offset.get_y(), offset.get_z(), block);
            if section.is_empty() {
                self.sections[y_index] = None;
            }
        } else {
            let mut section = ChunkSection::default();
            section.set_block_at_pos(offset.get_x(), offset.get_y(), offset.get_z(), block);
            self.sections[y_index] = Some(section);
        }
    }
}

impl Default for Chunk {
    fn default() -> Self {
        Chunk {
            sections: [
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None,
            ],
            position: ChunkPosition::new(0, 0),
        }
    }
}

pub struct ChunkPosition {
    x: i32,
    z: i32,
}
impl ChunkPosition {
    pub fn new(x: i32, z: i32) -> Self {
        Self { x, z }
    }
    pub fn get_x(&self) -> i32 {
        self.x
    }
    pub fn get_z(&self) -> i32 {
        self.z
    }
    /// Minecraft Source: ChunkCoordIntPair::a
    pub fn get_u64(&self) -> u64 {
        // Java source:
        // (long) x & 4294967295L | ((long) z & 4294967295L) << 32
        self.x as u64 | ((self.z as u64) << 32)
    }
    /// Minecraft Source: ChunkCoordIntPair(long i)
    pub fn from_u64(index: u64) -> Self {
        // Java source:
        // this.x = (int) i;
        // this.z = (int) (i >> 32);
        Self {
            x: index as i32,
            z: (index >> 32) as i32,
        }
    }
}
impl From<Vec3d<i32>> for ChunkPosition {
    fn from(from: Vec3d<i32>) -> ChunkPosition {
        ChunkPosition {
            x: (*from.get_x_as_ref()) >> 4,
            z: (*from.get_z_as_ref()) >> 4,
        }
    }
}
// https://wiki.vg/Data_Generators
#[derive(Clone)]
pub struct ChunkSection {
    sky_light: NibbleArray4096,
    emitted_light: NibbleArray4096,
    data: BitArray,
    palette: Option<Vec<u16>>,
    changed: bool,
    solid_blocks: u16,
}

impl ChunkSection {
    pub fn from_data(
        mut data: BitArray,
        mut palette: Option<Vec<u16>>,
        sky_light: NibbleArray4096,
        emitted_light: NibbleArray4096,
    ) -> Self {
        // ChunkSection must guarantee a sorted palette
        if let Some(palette) = palette.as_mut() {
            Self::sort_palette_data(&mut data, palette);
        }

        // Count all solid blocks
        let air_state = Block::Air.to_state() as u64;
        let mut solid_blocks = 0;
        for pos in 0..16 * 16 * 16 {
            if data.get(pos) != air_state {
                solid_blocks += 1;
            }
        }
        Self {
            sky_light,
            emitted_light,
            data,
            palette,
            changed: false,
            solid_blocks,
        }
    }
    fn sort_palette_data(data: &mut BitArray, palette: &mut Vec<u16>) {
        let original = palette.clone();
        palette.sort_unstable();
        for pos in 0..16 * 16 * 16 {
            let old_data = data.get(pos);
            let new_data = palette.binary_search(&original[old_data as usize]).unwrap();
            data.set(pos, new_data as u64);
        }
    }
    pub fn is_empty(&self) -> bool {
        self.solid_blocks == 0
    }
    pub fn get_block_at_pos(&self, x: u8, y: u8, z: u8) -> Block {
        let state = self.get_block_state_at_pos(x, y, z);
        Block::from_state(state).unwrap()
    }
    pub fn get_block_state_at_pos(&self, x: u8, y: u8, z: u8) -> usize {
        let palette_index = self.data.get_at_pos(x, y, z) as usize;
        match &self.palette {
            Some(palette) => palette[palette_index as usize] as usize, // Chunk palette
            None => palette_index,                                     // Global palette
        }
    }
    pub fn get_block_state_at(&self, pos: usize) -> usize {
        let palette_index = self.data.get(pos) as usize;
        match &self.palette {
            Some(palette) => palette[palette_index as usize] as usize, // Chunk palette
            None => palette_index,                                     // Global palette
        }
    }
    pub fn set_block_state_at(&mut self, pos: usize, state: usize) {
        self.changed = true;

        let palette_index: usize = if let Some(palette) = self.palette.as_mut() {
            // Find state in the palette
            match palette.binary_search(&(state as u16)) {
                Ok(index) => index,
                Err(index) => {
                    // Or add it to the palette
                    palette.insert(index, state as u16);
                    // Resize?
                    // Figure out the new bits per block in the block data array
                    let bpb = (64 - (palette.len() as u64 - 1).leading_zeros()) as u8;
                    let palette_index = if bpb <= BitArray::MAX_BITS_PER_ITEM {
                        self.data = self
                            .data
                            .resize_to(if bpb < BitArray::MIN_BITS_PER_ITEM {
                                BitArray::MIN_BITS_PER_ITEM
                            } else {
                                bpb
                            })
                            .unwrap();
                        index
                    } else {
                        // Use global palette
                        let mut new_data =
                            BitArray::new(block::blocks::USED_PALETTE_BITS, 16 * 16 * 16);
                        for pos in 0..16 * 16 * 16 {
                            let block = self.get_block_state_at(pos);
                            new_data.set(pos, block as u64);
                        }
                        self.palette = None;
                        self.data = new_data;
                        state as usize
                    };

                    // Correct data after palette insertion because it got offset
                    // # Example: (D was inserted) #
                    // ## Before ##
                    // Palette: [A] [B] [F] [H]
                    // Data: 2=F 0=A 1=B 2=F 0=A 3=H
                    // ## After ##
                    // Palette: [A] [B] [D] [F] [H]
                    // Data: 2=D 0=A 1=B 2=D 0=A 3=F
                    // ## Corrected ##
                    // Data: 3=F 0=A 1=B 3=F 0=A 4=H
                    for pos in 0..16 * 16 * 16 {
                        let entry = self.data.get(pos);
                        if entry as usize >= index {
                            self.data.set(pos, entry + 1);
                        }
                    }

                    palette_index
                }
            }
        } else {
            // Use global palette
            state as usize
        };

        let air_state = Block::Air.to_state();
        let old_state = self.get_block_state_at(pos);
        if old_state == air_state {
            if state != air_state {
                // Replaced air with solid
                self.solid_blocks += 1;
            }
        } else {
            if state == air_state {
                // Replaced solid with air
                self.solid_blocks -= 1;
            }
        }

        self.data.set(pos, palette_index as u64);
        debug_assert_eq!(self.get_block_state_at(pos), state);
    }
    pub fn set_block_at_pos(&mut self, x: u8, y: u8, z: u8, block: Block) {
        self.set_block_state_at(position_to_index_usize(x, y, z), block.to_state());
    }
    /// Recalculates the block palette and attempts to shrink the data array
    pub fn optimize(&mut self) -> bool {
        if !self.changed {
            return false;
        }
        self.changed = false;
        let mut new_palette: Vec<u16> = vec![];
        for pos in 0..16 * 16 * 16 {
            let block = self.get_block_state_at(pos) as u16;
            match new_palette.binary_search(&block) {
                Ok(_) => (),
                Err(index) => new_palette.insert(index, block),
            }
        }

        for pos in 0..16 * 16 * 16 {
            let block = self.get_block_state_at(pos) as u16;
            self.data
                .set(pos, new_palette.binary_search(&block).unwrap() as u64);
        }

        let mut optimized_bpb = (64 - (new_palette.len() as u64 - 1).leading_zeros()) as u8;

        if optimized_bpb > BitArray::MAX_BITS_PER_ITEM {
            // Global palette
            self.palette = None;
        } else {
            // Resize palette
            if optimized_bpb < BitArray::MIN_BITS_PER_ITEM {
                optimized_bpb = BitArray::MIN_BITS_PER_ITEM;
            }
            self.data = self.data.resize_to(optimized_bpb).unwrap();
            self.palette = Some(new_palette);
        }
        true
    }

    pub fn debug_dump_data(&self, file: String) {
        use std::fs::*;
        let mut buf = std::vec::Vec::new();
        for i in 0..self.data.len() {
            let pal = self.data.get(i);
            buf.append(&mut pal.to_string().into_bytes());
            buf.push(b";"[0]);
        }
        match write(file, buf) {
            Ok(()) => (),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}

impl Default for ChunkSection {
    fn default() -> Self {
        let air_state = Block::Air.to_state() as u16;
        Self {
            sky_light: NibbleArray4096::new(),
            emitted_light: NibbleArray4096::new(),
            data: BitArray::new(4, 16 * 16 * 16),
            palette: Some(vec![air_state]),
            changed: false,
            solid_blocks: 0,
        }
    }
}

fn position_to_index_usize(x: u8, y: u8, z: u8) -> usize {
    assert!(x < 16);
    assert!(y < 16);
    assert!(z < 16);
    ((y as usize) << 8) | ((z as usize) << 4) | x as usize
}
