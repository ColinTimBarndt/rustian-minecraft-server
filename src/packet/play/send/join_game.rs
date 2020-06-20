use crate::packet::{data::write, PacketSerialOut};
use crate::server::universe::{
    world::{Dimension, LevelType},
    Gamemode,
};

/// # Join Game
/// [Documentation](https://wiki.vg/Protocol#Join_Game)
///
/// See [Protocol Encryption](https://wiki.vg/Protocol_Encryption) for information on logging in.
pub struct JoinGame {
    pub entity_id: u32,
    pub gamemode: Gamemode,
    pub dimension: Dimension,
    pub seed_hash: u64,
    pub level_type: LevelType,
    pub view_distance: u32,
    pub reduced_debug_info: bool,
    pub show_respawn_screen: bool,
}

impl PacketSerialOut for JoinGame {
    const ID: u32 = 0x26;
    fn write(&self, buffer: &mut Vec<u8>) -> Result<(), String> {
        write::u32(buffer, self.entity_id);
        write::u8(buffer, self.gamemode as u8);
        write::i32(buffer, self.dimension as i32);
        write::u64(buffer, self.seed_hash);
        write::u8(buffer, 0); // Max Players (unused)
        write::string(buffer, self.level_type.to_string());
        write::var_u32(buffer, self.view_distance);
        write::bool(buffer, self.reduced_debug_info);
        write::bool(buffer, self.show_respawn_screen);
        Ok(())
    }
}
