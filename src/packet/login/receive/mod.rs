use std::error::Error;
use crate::packet::{PacketSerialIn,data::read};

#[derive(Debug)]
pub struct LoginStart {
    pub name: String
}

#[derive(Debug)]
pub struct EncryptionResponse {
    pub shared_secret: Vec<u8>,
    pub verify_token: Vec<u8>
}

#[derive(Debug)]
pub struct LoginPluginResponse {
    pub message_identifier: u32,
    pub successful: bool,
    pub data: Option<Vec<u8>>
}

impl PacketSerialIn for LoginStart {
    const ID: u32 = 0x00;
    fn consume_read(mut buffer: Vec<u8>) -> Result<LoginStart, Box<dyn Error>> {
        Ok(Self {
            name: read::string(&mut buffer)?
        })
    }
}

impl PacketSerialIn for EncryptionResponse {
    const ID: u32 = 0x01;
    fn consume_read(mut buffer: Vec<u8>) -> Result<EncryptionResponse, Box<dyn Error>> {
        let sl = read::var_u32(&mut buffer)? as usize;
        let s_secret = buffer.drain(0..sl).collect();

        let tl = read::var_u32(&mut buffer)? as usize;
        let v_token = buffer.drain(0..tl).collect();

        Ok(Self {
            shared_secret: s_secret,
            verify_token: v_token
        })
    }
}

impl PacketSerialIn for LoginPluginResponse {
    const ID: u32 = 0x02;
    fn consume_read(mut buffer: Vec<u8>) -> Result<LoginPluginResponse, Box<dyn Error>> {
        let m_id = read::var_u32(&mut buffer)?;
        let successful = read::bool(&mut buffer)?;
        Ok(Self {
            message_identifier: m_id,
            successful: successful,
            data: if buffer.len() > 0 {
                Some(buffer.drain(0..buffer.len()).collect())
            } else {
                None
            }
        })
    }
}
