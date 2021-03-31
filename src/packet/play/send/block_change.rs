use crate::helpers::Vec3d;
use crate::packet::{data::write, packet_ids::PLAY_CB_BLOCK_CHANGE, PacketSerialOut};
use crate::server::universe::world::Block;

/// # Block Change
/// [Documentation](https://wiki.vg/Protocol#Block_Change)
///
/// Fired whenever a block is changed within the render distance.
#[derive(Debug, Clone, Copy)]
pub struct BlockChange {
    pub position: Vec3d<i32>,
    pub block: usize,
}

impl PacketSerialOut for BlockChange {
    const ID: u32 = PLAY_CB_BLOCK_CHANGE;
    fn write(&self, buffer: &mut Vec<u8>) -> Result<(), String> {
        write::block_position(buffer, &self.position)?;
        write::var_usize(buffer, self.block);
        Ok(())
    }
}

impl BlockChange {
    pub fn new(position: Vec3d<i32>, block: Block) -> Self {
        Self {
            position,
            block: block.to_state(),
        }
    }
}
