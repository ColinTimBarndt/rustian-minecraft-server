use crate::helpers::chat_components::{ChatComponent, ChatComponentType};
use crate::packet::{data::write, PacketSerialOut};

/// # Disconnect (play)
/// [Documentation](https://wiki.vg/Protocol#Disconnect_.28play.29)
///
/// Sent by the server before it disconnects a client. The client assumes
/// that the server has already closed the connection by the time the packet arrives.
#[derive(Debug, Clone)]
pub struct Disconnect {
  pub reason: ChatComponent,
}

impl PacketSerialOut for Disconnect {
  const ID: u32 = 0x1B;
  fn write(&self, buffer: &mut Vec<u8>) -> Result<(), String> {
    write::json(buffer, &self.reason.make_json());
    Ok(())
  }
}

impl From<String> for Disconnect {
  fn from(msg: String) -> Self {
    Self {
      reason: ChatComponent::new(ChatComponentType::Text(msg)),
    }
  }
}

impl From<&str> for Disconnect {
  fn from(msg: &str) -> Self {
    Self {
      reason: ChatComponent::new(ChatComponentType::Text(msg.to_string())),
    }
  }
}

impl From<ChatComponent> for Disconnect {
  fn from(msg: ChatComponent) -> Self {
    Self { reason: msg }
  }
}
