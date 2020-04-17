use super::BlockWorld;

pub struct WorldHost<Generator: ChunkGenerator, Loader: ChunkLoader> {
    world: BlockWorld<Generator, Loader>
}