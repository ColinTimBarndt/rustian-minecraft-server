use crate::packet::{data::read, PacketSerialIn};
use std::error::Error;

#[derive(Debug)]
/// # Encryption Response
/// [Documentation](https://wiki.vg/Protocol#Encryption_Response)
///
/// See [Protocol Encryption](https://wiki.vg/Protocol_Encryption) for details.
pub struct EncryptionResponse {
  pub shared_secret: Vec<u8>,
  pub verify_token: Vec<u8>,
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
      verify_token: v_token,
    })
  }
}
