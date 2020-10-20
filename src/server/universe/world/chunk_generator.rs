use super::{Block, Chunk, ChunkPosition};
use crate::server::registries::Biome;

pub trait ChunkGenerator: Send + 'static {
    fn generate(&self, pos: ChunkPosition) -> Box<Chunk>;
    fn dyn_clone(&self) -> Box<dyn ChunkGenerator>;
}

impl<T> From<Box<T>> for Box<dyn ChunkGenerator>
where
    T: ChunkGenerator,
{
    fn from(from: Box<T>) -> Box<dyn ChunkGenerator> {
        from as Box<dyn ChunkGenerator>
    }
}

/// Generates chunks based on layers
#[derive(Clone)]
pub struct FlatWorldGenerator {
    layers: Vec<(Block, u8)>,
    biome: Biome,
}

impl FlatWorldGenerator {
    pub fn new(layers: &[(Block, u8)], biome: Biome) -> Self {
        Self {
            layers: layers.into(),
            biome,
        }
    }
}
impl ChunkGenerator for FlatWorldGenerator {
    fn generate(&self, pos: ChunkPosition) -> Box<Chunk> {
        use crate::helpers::Vec3d;
        let mut chunk = Box::new(Chunk::new_empty(pos));
        let mut y = 0u8;
        for (block, height) in self.layers.iter() {
            fill_slice(&mut chunk, y, y + height, *block);
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
    fn dyn_clone(&self) -> Box<dyn ChunkGenerator> {
        Box::new(Clone::clone(self)).into()
    }
}

/// Generates all chunks by copying the template
#[derive(Clone)]
pub struct TemplateChunkGenerator {
    template: Box<Chunk>,
}

impl TemplateChunkGenerator {
    pub fn new(template: Box<Chunk>) -> Self {
        Self { template }
    }
}
impl ChunkGenerator for TemplateChunkGenerator {
    fn generate(&self, pos: ChunkPosition) -> Box<Chunk> {
        Box::new(self.template.duplicate(pos))
    }
    fn dyn_clone(&self) -> Box<dyn ChunkGenerator> {
        Box::new(Clone::clone(self)).into()
    }
}
