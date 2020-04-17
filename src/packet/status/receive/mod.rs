use std::error::Error;
use crate::packet::{PacketSerialIn,data::read};

#[derive(Debug)]
pub struct Request {}

#[derive(Debug)]
pub struct Ping {
    pub payload: u64
}

impl PacketSerialIn for Request {
    const ID: u32 = 0x00;
    fn read(_buffer: &mut Vec<u8>) -> Result<Request, Box<dyn Error>> {
        Ok(Request {})
    }
    fn consume_read(_buffer: Vec<u8>) -> Result<Request, Box<dyn Error>> {
        Ok(Request {})
    }
}

impl PacketSerialIn for Ping {
    const ID: u32 = 0x01;
    fn consume_read(mut buffer: Vec<u8>) -> Result<Ping, Box<dyn Error>> {
        Ok(Ping {
            payload: read::u64(&mut buffer)?
        })
    }
}
