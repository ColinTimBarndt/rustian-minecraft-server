use super::super::send;
use crate::packet::{
  data::read, packet_ids::PLAY_SB_KEEP_ALIVE, PacketParsingError, PacketSerialIn,
};

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
  const ID: u32 = PLAY_SB_KEEP_ALIVE;
  fn read(buffer: &mut &[u8]) -> Result<Self, PacketParsingError> {
    Ok(Self {
      keep_alive_id: read::u64(buffer)?,
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
