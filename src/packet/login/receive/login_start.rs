use crate::packet::{
  data::read, packet_ids::LOGIN_SB_LOGIN_START, PacketParsingError, PacketSerialIn,
};

#[derive(Debug)]
/// # Login Start
/// [Documentation](https://wiki.vg/Protocol#Login_Start)
pub struct LoginStart {
  /// Player's username
  pub name: String,
}

impl PacketSerialIn for LoginStart {
  const ID: u32 = LOGIN_SB_LOGIN_START;
  fn read(buffer: &mut &[u8]) -> Result<LoginStart, PacketParsingError> {
    Ok(Self {
      name: read::string(buffer)?,
    })
  }
}
