use crate::packet::{
    packet_ids::HANDSHAKE_SB_HANDSHAKE, PacketParsingError, PacketSerialIn, PlayerConnectionState,
};

#[derive(Debug)]
pub struct Handshake {
    pub version: u32,
    pub address: String,
    pub port: u16,
    pub next_state: PlayerConnectionState,
}

impl PacketSerialIn for Handshake {
    const ID: u32 = HANDSHAKE_SB_HANDSHAKE;
    fn read(buffer: &mut &[u8]) -> Result<Self, PacketParsingError> {
        use crate::packet::data::read;

        let ver = read::var_i32(buffer)?;
        let addrstr = read::string(buffer)?;
        let port = read::u16(buffer)?;
        let next = read::var_i32(buffer)?;

        Ok(Handshake {
            version: ver as u32,
            address: addrstr,
            port: port,
            next_state: match next {
                1 => PlayerConnectionState::Status,
                2 => PlayerConnectionState::Login,
                x => {
                    return Err(PacketParsingError::InvalidPacket(format!(
                        "Invalid next state: {}",
                        x
                    )))
                }
            },
        })
    }
}
