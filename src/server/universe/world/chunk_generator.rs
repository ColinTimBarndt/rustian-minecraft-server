use super::{Block, Chunk, ChunkPosition};

pub trait ChunkGenerator: Send + 'static {
    fn generate(&mut self, pos: ChunkPosition) -> Chunk;
}

/// Generates chunks based on layers
pub struct FlatWorldGenerator {
    layers: Vec<(Block, u8)>,
}

impl FlatWorldGenerator {
    pub fn new(layers: Vec<(Block, u8)>) -> Self {
        Self { layers }
    }
}
impl ChunkGenerator for FlatWorldGenerator {
    fn generate(&mut self, pos: ChunkPosition) -> Chunk {
        use crate::helpers::Vec3d;
        let mut chunk = Chunk::new_empty(pos);
        let mut y = 0u8;
        for (block, height) in self.layers.iter() {
            fill_slice(&mut chunk, y, y + height, block.clone());
            y += height;
        }
        return chunk;
        #[inline]
        fn fill_slice(chunk: &mut Chunk, y1: u8, y2: u8, block: Block) {
            for x in 0..16u8 {
                for z in 0..16u8 {
                    for y in y1..y2 {
                        chunk.set_block_at_pos(Vec3d::new(x, y, z), block);
                    }
                }
            }
        }
    }
}
