use super::{Chunk, ChunkPosition, block::Block};

pub trait ChunkGenerator {
    fn generate(pos: ChunkPosition) -> Chunk;
}

/// Generates chunks based on layers
pub struct FlatWorldGenerator {
    layers: Vec<(Block, u8)>
}

impl FlatWorldGenerator {
    pub fn new(layers: Vec<(Block, u8)>) -> Self {
        Self {
            layers
        }
    }
}
impl ChunkGenerator for FlatWorldGenerator {
    fn generate(pos: ChunkPosition) -> Chunk {
        let chunk = Chunk::new_empty(pos);

        chunk
    }
}