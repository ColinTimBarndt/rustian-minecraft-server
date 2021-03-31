use crate::packet::{
    data::read, packet_ids::PLAY_SB_CHAT_MESSAGE, PacketParsingError, PacketSerialIn,
};

/// # Chat Message (serverbound)
/// [Documentation](https://wiki.vg/Protocol#Chat_Message_.28serverbound.29)
///
/// Used to send a chat message to the server. The message may not be longer
/// than 256 characters or else the server will kick the client.
#[derive(Debug, Clone)]
pub struct ChatMessage {
    pub message: String,
}

impl PacketSerialIn for ChatMessage {
    const ID: u32 = PLAY_SB_CHAT_MESSAGE;
    fn read(buffer: &mut &[u8]) -> Result<Self, PacketParsingError> {
        let str = read::string(buffer)?;
        match str.len() {
            0..=256 => Ok(Self { message: str }),
            _ => Err(PacketParsingError::InvalidPacket(
                "Chat message length not allowed".into(),
            )),
        }
    }
}

impl ChatMessage {
    #[inline]
    pub fn is_command(&self) -> bool {
        if let Some(b'/') = self.message.bytes().next() {
            true
        } else {
            false
        }
    }
}
