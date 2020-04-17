use std::error::Error;
use crate::packet::{PacketSerialIn, PacketParsingError, PlayerConnectionState};

#[derive(Debug)]
pub struct Handshake {
    pub version: u32,
    pub address: String,
    pub port: u16,
    pub next_state: PlayerConnectionState
}

impl PacketSerialIn for Handshake {
    const ID: u32 = 0x00;
    fn consume_read(mut buffer: Vec<u8>) -> Result<Self, Box<dyn Error>> {
        use crate::packet::data::read;

        let ver = read::var_i32(&mut buffer)?;
        let addrstr = read::string(&mut buffer)?;
        let port = read::u16(&mut buffer)?;
        let next = read::var_i32(&mut buffer)?;

        Ok(Handshake {
            version: ver as u32,
            address: addrstr,
            port: port,
            next_state: match next {
                1 => PlayerConnectionState::Status,
                2 => PlayerConnectionState::Login,
                x => return Err(Box::new(PacketParsingError::InvalidPacket(
                    format!("Invalid next state: {}", x)
                )))
            }
        })
    }
}
