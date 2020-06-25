use crate::packet::{data::write, PacketSerialOut};

#[derive(Debug, Clone)]
pub struct SetCompression {
  pub threshold: u32,
}

impl PacketSerialOut for SetCompression {
  const ID: u32 = 0x03;
  fn write(&self, buffer: &mut Vec<u8>) -> Result<(), String> {
    (*self).clone().consume_write(buffer)
  }
  fn consume_write(self, buffer: &mut Vec<u8>) -> Result<(), String> {
    if self.threshold > 0x7f_ff_ff_ff {
      panic!("Compression threshold too big for the serializer");
    }
    write::var_u32(buffer, self.threshold);
    Ok(())
  }
}