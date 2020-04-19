use crate::helpers::Vec3d;
use std::collections::HashMap;

pub mod block;
mod dimension;
pub use block::Block;
pub use dimension::*;
mod level_type;
pub use level_type::*;
pub mod chunk;
pub mod chunk_generator;
pub mod chunk_loader;
pub use chunk::*;
pub mod region;
pub use region::*;

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
