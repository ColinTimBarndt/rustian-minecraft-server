use crate::helpers::chat_components::ChatComponent;
use crate::packet::{data::write, PacketSerialOut};

/// # Disconnect (play)
/// [Documentation](https://wiki.vg/Protocol#Disconnect_.28play.29)
///
/// Sent by the server before it disconnects a client. The client assumes
/// that the server has already closed the connection by the time the packet arrives.
#[derive(Debug, Clone)]
pub struct Disconnect<'a> {
  pub reason: &'a [ChatComponent],
}

impl PacketSerialOut for Disconnect<'_> {
  const ID: u32 = 0x1B;
  fn write(&self, buffer: &mut Vec<u8>) -> Result<(), String> {
    write::chat_components(buffer, self.reason);
    Ok(())
  }
}

impl<'a: 'b, 'b> From<&'a ChatComponent> for Disconnect<'b> {
  fn from(msg: &'a ChatComponent) -> Self {
    Self {
      reason: std::slice::from_ref(msg),
    }
  }
}

impl<'a: 'b, 'b> From<&'a [ChatComponent]> for Disconnect<'b> {
  fn from(msg: &'a [ChatComponent]) -> Self {
    Self { reason: msg }
  }
}
