use crate::packet::{
    PacketSerialIn, data::read, PacketParsingError
};
use crate::server::universe::{PlayerChatMode, DisplayedPlayerModelParts, PlayerHand};

pub struct ClientSettings {
    pub locale: String,
    pub view_distance: u8,
    pub chat_mode: PlayerChatMode,
    pub chat_colors: bool,
    pub displayed_model_parts: DisplayedPlayerModelParts,
    pub main_hand: PlayerHand
}

impl PacketSerialIn for ClientSettings {
    const ID: u32 = 0x05;
    fn consume_read(mut buffer: Vec<u8>) -> Result<Self, Box<dyn std::error::Error>> {
        use num_traits::FromPrimitive;
        Ok(Self {
            locale: read::string(&mut buffer)?,
            view_distance: read::u8(&mut buffer)?,
            chat_mode: match FromPrimitive::from_u32(read::var_u32(&mut buffer)?) {
                Some(x) => x,
                None => return Err(Box::new(PacketParsingError::InvalidPacket(String::from(
                    format!("Invalid Chat Mode in ClientSettings packet")
                ))))
            },
            chat_colors: read::bool(&mut buffer)?,
            displayed_model_parts: DisplayedPlayerModelParts::new(read::u8(&mut buffer)?),
            main_hand: match FromPrimitive::from_u32(read::var_u32(&mut buffer)?) {
                Some(x) => x,
                None => return Err(Box::new(PacketParsingError::InvalidPacket(String::from(
                    format!("Invalid Main Hand in ClientSettings packet")
                ))))
            }
        })
    }
}

impl std::fmt::Display for ClientSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(
            f,
            "locale={}, view_distance={}, chat_mode={}, chat_colors={}, displayed_model_parts={}, main_hand={}",
            self.locale,
            self.view_distance,
            self.chat_mode,
            self.chat_colors,
            self.displayed_model_parts,
            self.main_hand
        )
    }
}