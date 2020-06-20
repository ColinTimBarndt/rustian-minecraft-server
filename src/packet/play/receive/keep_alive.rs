use super::super::send;
use crate::packet::{data::read, PacketSerialIn};

/// # Keep Alive (serverbound)
/// [Documentation](https://wiki.vg/Protocol#Keep_Alive_.28serverbound.29)
///
/// The server will frequently send out a keep-alive, each containing a
/// random ID. The client must respond with the same packet.
#[derive(Clone, Debug)]
pub struct KeepAlive {
  pub keep_alive_id: u64,
}

impl PacketSerialIn for KeepAlive {
  const ID: u32 = 0x0F;
  fn consume_read(mut buffer: Vec<u8>) -> Result<Self, Box<dyn std::error::Error>> {
    Ok(Self {
      keep_alive_id: read::u64(&mut buffer)?,
    })
  }
}

impl From<send::KeepAlive> for KeepAlive {
  fn from(send: send::KeepAlive) -> Self {
    Self {
      keep_alive_id: send.keep_alive_id,
    }
  }
}
