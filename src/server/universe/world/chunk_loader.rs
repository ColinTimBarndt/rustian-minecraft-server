use super::{Chunk, ChunkPosition};

pub trait ChunkLoader {
    fn load(&mut self, pos: ChunkPosition) -> Chunk;
}

/// Generates all chunks by copying the template
pub struct TemplateChunkLoader {
    template: Chunk
}

impl TemplateChunkLoader {
    pub fn new(template: Chunk) -> Self {
        Self {
            template
        }
    }
}
impl ChunkLoader for TemplateChunkLoader {
    fn load(&mut self, pos: ChunkPosition) -> Chunk {
        self.template.copy(pos)
    }
}