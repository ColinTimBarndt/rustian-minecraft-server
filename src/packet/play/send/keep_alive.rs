use super::super::receive;
use crate::packet::{data::write, PacketSerialOut};

/// [Documentation](https://wiki.vg/Protocol#Keep_Alive_.28clientbound.29)
#[derive(Clone, Debug)]
pub struct KeepAlive {
  pub keep_alive_id: u64,
}

impl PacketSerialOut for KeepAlive {
  const ID: u32 = 0x21;
  fn write(&self, buffer: &mut Vec<u8>) -> Result<(), String> {
    write::u64(buffer, self.keep_alive_id);
    Ok(())
  }
}

impl From<receive::KeepAlive> for KeepAlive {
  fn from(recv: receive::KeepAlive) -> Self {
    Self {
      keep_alive_id: recv.keep_alive_id,
    }
  }
}
