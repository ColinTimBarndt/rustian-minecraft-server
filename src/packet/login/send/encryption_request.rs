use crate::packet::{data::write, PacketSerialOut};

#[derive(Debug, Clone)]
pub struct EncryptionRequest {
  pub server_identifier: String,
  pub public_key: Vec<u8>,
  pub verify_token: Vec<u8>,
}

impl EncryptionRequest {
  pub fn new(sid: String, p_key: Vec<u8>, verify_sec: u8) -> Self {
    use rand::RngCore;
    let mut rng = rand::thread_rng();
    let mut v_token = Vec::with_capacity((verify_sec * 4) as usize);
    for _ in 0..verify_sec {
      let n = rng.next_u64();
      v_token.push(n as u8);
      v_token.push((n >> 8) as u8);
      v_token.push((n >> 16) as u8);
      v_token.push((n >> 24) as u8);
    }
    Self {
      server_identifier: sid,
      public_key: p_key,
      verify_token: v_token,
    }
  }
}

impl PacketSerialOut for EncryptionRequest {
  const ID: u32 = 0x01;
  fn write(&self, buffer: &mut Vec<u8>) -> Result<(), String> {
    (*self).clone().consume_write(buffer)
  }
  fn consume_write(mut self, buffer: &mut Vec<u8>) -> Result<(), String> {
    write::string(buffer, self.server_identifier);

    write::var_u32(buffer, self.public_key.len() as u32);
    buffer.append(&mut self.public_key);

    write::var_u32(buffer, self.verify_token.len() as u32);
    buffer.append(&mut self.verify_token);

    Ok(())
  }
}
