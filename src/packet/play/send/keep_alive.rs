use super::super::receive;
use crate::packet::{data::write, packet_ids::PLAY_CB_KEEP_ALIVE, PacketSerialOut};

/// # Keep Alive (clientbound)
/// [Documentation](https://wiki.vg/Protocol#Keep_Alive_.28clientbound.29)
///
/// The server will frequently send out a keep-alive, each containing a random ID.
/// The client must respond with the same packet. If the client does not respond to
/// them for over 30 seconds, the server kicks the client. Vice versa, if the server
/// does not send any keep-alives for 20 seconds, the client will disconnect and
/// yields a "Timed out" exception.
///
/// The Notchian server uses a system-dependent time in milliseconds to generate the
/// keep alive ID value.
#[derive(Clone, Debug)]
pub struct KeepAlive {
  pub keep_alive_id: u64,
}

impl PacketSerialOut for KeepAlive {
  const ID: u32 = PLAY_CB_KEEP_ALIVE;
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
