use crate::packet::{data::read, PacketParsingError, PacketSerialIn};

#[derive(Debug)]
pub struct Request {}

#[derive(Debug)]
pub struct Ping {
    pub payload: u64,
}

impl PacketSerialIn for Request {
    const ID: u32 = 0x00;
    fn read(_buffer: &mut &[u8]) -> Result<Request, PacketParsingError> {
        Ok(Request {})
    }
}

impl PacketSerialIn for Ping {
    const ID: u32 = 0x01;
    fn read(buffer: &mut &[u8]) -> Result<Ping, PacketParsingError> {
        Ok(Ping {
            payload: read::u64(buffer)?,
        })
    }
}
