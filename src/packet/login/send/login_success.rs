use crate::packet::{data::write, PacketSerialOut};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct LoginSuccess {
  pub uuid: Uuid,
  pub username: String,
}

impl PacketSerialOut for LoginSuccess {
  const ID: u32 = 0x02;
  fn write(&self, buffer: &mut Vec<u8>) -> Result<(), String> {
    write::string(buffer, self.uuid.hyphenated().to_string());
    write::string(buffer, self.username.clone());
    Ok(())
  }
  fn consume_write(self, buffer: &mut Vec<u8>) -> Result<(), String> {
    write::string(buffer, self.uuid.hyphenated().to_string());
    write::string(buffer, self.username);
    Ok(())
  }
}
