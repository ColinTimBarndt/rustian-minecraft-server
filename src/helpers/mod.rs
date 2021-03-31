mod bit_array;
pub mod chat_components;
mod chunk_send_order;
mod color;
mod namespaced_key;
mod nibble_array;
mod registry;
mod rotation;
pub mod vector;

pub use bit_array::*;
pub use chunk_send_order::*;
pub use color::*;
pub use namespaced_key::*;
pub use nibble_array::*;
pub use registry::*;
pub use rotation::*;
pub use vector::Vec3d;
pub mod fast;
pub mod mojang_api;

#[test]
fn test_chunk_send_order() {
    use crate::server::universe::world::chunk::ChunkPosition;
    let center = ChunkPosition::new(2, 1);
    let result = get_chunks_to_load(3, center, |c| *c == ChunkPosition::new(2, 2));
    for pos in &result {
        println!("Pos: {:?}", pos);
    }
    println!("Length: {}", result.len());
}
