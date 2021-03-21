use crate::packet::{data::write, packet_ids::LOGIN_CB_LOGIN_SUCCESS, PacketSerialOut};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct LoginSuccess {
    pub uuid: Uuid,
    pub username: String,
}

impl PacketSerialOut for LoginSuccess {
    const ID: u32 = LOGIN_CB_LOGIN_SUCCESS;
    fn write(&self, buffer: &mut Vec<u8>) -> Result<(), String> {
        write::string(buffer, &self.uuid.to_hyphenated().to_string());
        write::string(buffer, &self.username);
        Ok(())
    }
    fn consume_write(self, buffer: &mut Vec<u8>) -> Result<(), String> {
        write::string(buffer, &self.uuid.to_hyphenated().to_string());
        write::string(buffer, &self.username);
        Ok(())
    }
}
