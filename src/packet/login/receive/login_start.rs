use crate::packet::{data::read, PacketSerialIn};
use std::error::Error;

#[derive(Debug)]
/// # Login Start
/// [Documentation](https://wiki.vg/Protocol#Login_Start)
pub struct LoginStart {
  /// Player's username
  pub name: String,
}

impl PacketSerialIn for LoginStart {
  const ID: u32 = 0x00;
  fn consume_read(mut buffer: Vec<u8>) -> Result<LoginStart, Box<dyn Error>> {
    Ok(Self {
      name: read::string(&mut buffer)?,
    })
  }
}
