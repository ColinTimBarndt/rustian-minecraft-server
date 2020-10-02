use crate::helpers::chat_components::ChatComponent;
use crate::packet::{data::write, PacketSerialOut};

/// [Documentation](https://wiki.vg/Protocol#Disconnect_.28login.29)
#[derive(Debug, Clone)]
pub struct Disconnect<'a> {
  pub reason: &'a [ChatComponent],
}

impl PacketSerialOut for Disconnect<'_> {
  const ID: u32 = 0x00;
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
