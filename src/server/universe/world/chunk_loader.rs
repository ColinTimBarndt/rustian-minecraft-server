use super::{Chunk, ChunkPosition};

pub trait ChunkLoader: Send + 'static {
    /// Returns Some(Chunk) if the chunk could be loaded.
    /// If None is returned, then the chunk should be
    /// generated instead.
    fn load_chunk(&mut self, pos: ChunkPosition) -> Option<Chunk>;
}

/// Generates all chunks by copying the template
pub struct TemplateChunkLoader {
    template: Chunk,
}

impl TemplateChunkLoader {
    pub fn new(template: Chunk) -> Self {
        Self { template }
    }
}
impl ChunkLoader for TemplateChunkLoader {
    fn load_chunk(&mut self, pos: ChunkPosition) -> Option<Chunk> {
        Some(self.template.copy(pos))
    }
}

// TODO: FileChunkLoader
