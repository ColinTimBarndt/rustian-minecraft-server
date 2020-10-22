use crate::packet::{
    data::read,
    packet_ids::{STATUS_SB_PING, STATUS_SB_REQUEST},
    PacketParsingError, PacketSerialIn,
};

#[derive(Debug)]
pub struct Request {}

#[derive(Debug)]
pub struct Ping {
    pub payload: u64,
}

impl PacketSerialIn for Request {
    const ID: u32 = STATUS_SB_REQUEST;
    fn read(_buffer: &mut &[u8]) -> Result<Request, PacketParsingError> {
        Ok(Request {})
    }
}

impl PacketSerialIn for Ping {
    const ID: u32 = STATUS_SB_PING;
    fn read(buffer: &mut &[u8]) -> Result<Ping, PacketParsingError> {
        Ok(Ping {
            payload: read::u64(buffer)?,
        })
    }
}
