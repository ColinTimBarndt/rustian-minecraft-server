use crate::packet::{data::read, PacketParsingError, PacketSerialIn};
use crate::server::universe::entity::player::{
    DisplayedPlayerModelParts, PlayerChatMode, PlayerHand, PlayerSettings,
};

/// # Client Settings
/// [Documentation](https://wiki.vg/Protocol#Client_Settings)
///
/// Sent when the player connects, or when settings are changed.
pub struct ClientSettings {
    pub settings: PlayerSettings,
}

impl PacketSerialIn for ClientSettings {
    const ID: u32 = 0x05;
    fn read(buffer: &mut &[u8]) -> Result<Self, PacketParsingError> {
        use num_traits::FromPrimitive;
        let locale = read::string(buffer)?;
        let view_distance = read::u8(buffer)? as u16; // Maybe graphics will become better
        let chat_mode = match FromPrimitive::from_u32(read::var_u32(buffer)?) {
            Some(x) => x,
            None => {
                return Err(PacketParsingError::InvalidPacket(String::from(format!(
                    "Invalid Chat Mode in ClientSettings packet"
                ))))
            }
        };
        let chat_colors_enabled = read::bool(buffer)?;
        let displayed_model_parts = DisplayedPlayerModelParts::new(read::u8(buffer)?);
        let main_hand = match FromPrimitive::from_u32(read::var_u32(buffer)?) {
            Some(x) => x,
            None => {
                return Err(PacketParsingError::InvalidPacket(String::from(format!(
                    "Invalid Main Hand in ClientSettings packet"
                ))))
            }
        };

        Ok(Self {
            settings: PlayerSettings {
                locale,
                view_distance,
                chat_mode,
                chat_colors_enabled,
                displayed_model_parts,
                main_hand,
            },
        })
    }
}

impl std::fmt::Display for ClientSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self.settings)
    }
}
