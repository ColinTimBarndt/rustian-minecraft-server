// Based on: https://github.com/feather-rs/feather/blob/develop/core/src/chunk.rs

use super::{blocks, Block};
use crate::helpers::{BitArray, NibbleArray4096, Registry, Vec3d};
use crate::server::registries::Biome;
use std::{fmt, ops};

pub const CHUNK_BLOCK_WIDTH: u32 = 16;
const COORDINATE_SCALE: i32 = CHUNK_BLOCK_WIDTH as i32;

pub const LOCAL_PALETTE_BIT_LIMIT: u8 = 9;

#[derive(Clone)]
pub struct Chunk {
    pub sections: [Option<ChunkSection>; 16],
    pub position: ChunkPosition,
    pub sky_light: [LightSection; 18],
    pub emitted_light: [LightSection; 18],
    biomes: [u32; 1024],
}

#[derive(Clone, Copy)]
pub enum LightSection {
    None,
    Zero,
    Some(NibbleArray4096),
}

impl fmt::Debug for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Chunk")
            .field("position", &self.position)
            .field("sections", &self.sections)
            .finish()
    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "Chunk({}, {})", self.position.x, self.position.z)
    }
}

impl Chunk {
    pub fn new_empty(pos: ChunkPosition) -> Self {
        Self {
            position: pos,
            ..Default::default()
        }
    }
    pub fn duplicate(&self, to: ChunkPosition) -> Self {
        Self {
            sections: self.sections.clone(),
            position: to,
            ..Default::default()
        }
    }
    pub fn get_block_at_pos(&self, offset: Vec3d<u8>) -> Block {
        let y_index = (offset.y >> 4) as usize;
        let section = &self.sections[y_index];
        match section {
            Some(section) => section.get_block_at_pos(offset.x, offset.y % 16, offset.z),
            None => Block::Air,
        }
    }
    pub fn set_block_at_pos(&mut self, offset: Vec3d<u8>, block: Block) {
        let y_index = (offset.y >> 4) as usize;
        let section = &mut self.sections[y_index];
        if let Some(section) = section.as_mut() {
            section.set_block_at_pos(offset.x, offset.y % 16, offset.z, block);
            if section.is_empty() {
                self.sections[y_index] = None;
            }
        } else {
            let mut section = ChunkSection::default();
            section.set_block_at_pos(offset.x, offset.y % 16, offset.z, block);
            self.sections[y_index] = Some(section);
        }
    }
    /// Sets a biome at a specific biome volume. Each volume
    /// has a size of 4*4*4 blocks. Each chunk section has 4
    /// by 4 by 4 volumes.
    pub fn set_biome_volume_at(&mut self, offset: Vec3d<u16>, biome: Biome) {
        assert!((0..CHUNK_BLOCK_WIDTH as u16 / 4).contains(&offset.x));
        assert!((0..256 / 4).contains(&offset.y));
        assert!((0..CHUNK_BLOCK_WIDTH as u16 / 4).contains(&offset.z));
        self.biomes[(offset.x + 4 * offset.z + 16 * offset.y) as usize] = biome.get_id() as u32;
    }
    pub fn get_biome_volume_at(&self, offset: Vec3d<u16>) -> Biome {
        assert!((0..CHUNK_BLOCK_WIDTH as u16 / 4).contains(&offset.x));
        assert!((0..256 / 4).contains(&offset.y));
        assert!((0..CHUNK_BLOCK_WIDTH as u16 / 4).contains(&offset.z));
        Biome::from_id(self.biomes[(offset.x + 4 * offset.z + 16 * offset.y) as usize] as usize)
            .unwrap()
    }
    pub fn get_biome_data(&self) -> &[u32; 1024] {
        &self.biomes
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
            sky_light: [LightSection::None; 18],
            emitted_light: [LightSection::None; 18],
            biomes: [Biome::Plains.get_id() as u32; 1024],
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, Eq)]
pub struct ChunkPosition {
    pub x: i32,
    pub z: i32,
}
impl ChunkPosition {
    pub fn new(x: i32, z: i32) -> Self {
        Self { x, z }
    }
    pub fn get_offset(&self) -> Vec3d<i32> {
        Vec3d::new(self.x * COORDINATE_SCALE, 0, self.z * COORDINATE_SCALE)
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
            x: (from.x) >> 4,
            z: (from.z) >> 4,
        }
    }
}
impl From<Vec3d<f64>> for ChunkPosition {
    fn from(from: Vec3d<f64>) -> ChunkPosition {
        ChunkPosition {
            x: (from.x.round() as i32) >> 4,
            z: (from.z.round() as i32) >> 4,
        }
    }
}

impl ops::Add<Self> for ChunkPosition {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            z: self.z + other.z,
        }
    }
}
impl ops::Sub<Self> for ChunkPosition {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            z: self.z - other.z,
        }
    }
}
impl ops::Add<&Self> for ChunkPosition {
    type Output = Self;
    fn add(self, other: &Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            z: self.z + other.z,
        }
    }
}
impl ops::Sub<&Self> for ChunkPosition {
    type Output = Self;
    fn sub(self, other: &Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            z: self.z - other.z,
        }
    }
}
impl ops::Add<Self> for &ChunkPosition {
    type Output = ChunkPosition;
    fn add(self, other: Self) -> Self::Output {
        ChunkPosition {
            x: self.x + other.x,
            z: self.z + other.z,
        }
    }
}
impl ops::Sub<Self> for &ChunkPosition {
    type Output = ChunkPosition;
    fn sub(self, other: Self) -> Self::Output {
        ChunkPosition {
            x: self.x - other.x,
            z: self.z - other.z,
        }
    }
}
impl ops::Add<&Self> for &ChunkPosition {
    type Output = ChunkPosition;
    fn add(self, other: &Self) -> Self::Output {
        ChunkPosition {
            x: self.x + other.x,
            z: self.z + other.z,
        }
    }
}
impl ops::Sub<&Self> for &ChunkPosition {
    type Output = ChunkPosition;
    fn sub(self, other: &Self) -> Self::Output {
        ChunkPosition {
            x: self.x - other.x,
            z: self.z - other.z,
        }
    }
}
impl std::cmp::PartialEq for ChunkPosition {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.z == other.z
    }
}

#[derive(Clone)]
pub struct ChunkSection {
    pub sky_light: NibbleArray4096,
    pub emitted_light: NibbleArray4096,
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
                    let palette_index = if bpb <= LOCAL_PALETTE_BIT_LIMIT
                    /*BitArray::MAX_BITS_PER_ITEM*/
                    {
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
                        let mut new_data = BitArray::new(blocks::USED_PALETTE_BITS, 16 * 16 * 16);
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

        if optimized_bpb > LOCAL_PALETTE_BIT_LIMIT
        /*BitArray::MAX_BITS_PER_ITEM*/
        {
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

    pub fn get_solid_block_count(&self) -> u16 {
        self.solid_blocks
    }

    pub fn get_palette(&self) -> &Option<Vec<u16>> {
        &self.palette
    }

    pub fn get_raw_block_data(&self) -> &BitArray {
        &self.data
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

impl std::fmt::Debug for ChunkSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ChunkSection")
            .field("solid_blocks", &self.solid_blocks)
            .field("changed", &self.changed)
            .finish()
    }
}

fn position_to_index_usize(x: u8, y: u8, z: u8) -> usize {
    assert!(x < 16);
    assert!(y < 16);
    assert!(z < 16);
    ((y as usize) << 8) | ((z as usize) << 4) | x as usize
}

#[cfg(test)]
mod tests {
    use super::blocks::BarrelData;
    use super::*;
    #[test]
    fn chunk_test() {
        let mut chunk = Chunk::new_empty(ChunkPosition::new(0, 0));

        {
            let mut b_data: BarrelData = Default::default();
            b_data.open = true;
            println!("Before: {:#?}", b_data);
            chunk.set_block_at_pos(Vec3d::new(5, 18, 2), Block::Barrel(b_data));
        }
        {
            let block = chunk.get_block_at_pos(Vec3d::new(5, 18, 2));
            println!("After: {:#?}", block);
            if let Block::Barrel(b_data) = block {
                assert_eq!(b_data.open, true);
            } else {
                panic!("Incorrect Block");
            }
        }
    }
}
