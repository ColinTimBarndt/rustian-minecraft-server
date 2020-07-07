use crate::packet::{data::read, PacketParsingError, PacketSerialIn};

#[derive(Debug)]
/// # Login Start
/// [Documentation](https://wiki.vg/Protocol#Login_Start)
pub struct LoginStart {
  /// Player's username
  pub name: String,
}

impl PacketSerialIn for LoginStart {
  const ID: u32 = 0x00;
  fn read(buffer: &mut &[u8]) -> Result<LoginStart, PacketParsingError> {
    Ok(Self {
      name: read::string(buffer)?,
    })
  }
}
