use super::{Chunk, ChunkPosition};

pub trait ChunkLoader: Send + 'static {
    /// Returns Some(Chunk) if the chunk could be loaded.
    /// If None is returned, then the chunk should be
    /// generated instead.
    fn load_chunk(&mut self, pos: ChunkPosition) -> Option<Chunk>;
}

impl<T> From<Box<T>> for Box<dyn ChunkLoader>
where
    T: ChunkLoader,
{
    fn from(from: Box<T>) -> Box<dyn ChunkLoader> {
        from as Box<dyn ChunkLoader>
    }
}

/// Can't load any chunks, but forces them to be generated
/// every time.
pub struct VoidChunkLoader {}

impl VoidChunkLoader {
    pub fn new() -> Self {
        Self {}
    }
}
impl ChunkLoader for VoidChunkLoader {
    fn load_chunk(&mut self, _pos: ChunkPosition) -> Option<Chunk> {
        None
    }
}

// TODO: FileChunkLoader
