use crate::helpers::chat_components::ChatComponent;
use crate::packet::{data::write, PacketSerialOut};
use json::JsonValue;

/// [Documentation](https://wiki.vg/Protocol#Chat_Message_.28clientbound.29)
#[derive(Clone, Debug)]
pub struct ChatMessage {
  pub message: Vec<ChatComponent>,
  pub message_type: ChatMessageType,
}

impl ChatMessage {
  pub fn from_single_component(msg: ChatComponent, msg_type: ChatMessageType) -> Self {
    Self {
      message: vec![msg],
      message_type: msg_type,
    }
  }
  pub fn from_components(msg: Vec<ChatComponent>, msg_type: ChatMessageType) -> Self {
    Self {
      message: msg,
      message_type: msg_type,
    }
  }
}

impl PacketSerialOut for ChatMessage {
  const ID: u32 = 0x0F;
  fn write(&self, buffer: &mut Vec<u8>) -> Result<(), String> {
    write::json(
      buffer,
      &JsonValue::Array(self.message.iter().map(|part| part.make_json()).collect()),
    );
    write::u8(buffer, self.message_type as u8);
    Ok(())
  }
}

#[derive(Copy, Clone, Debug)]
pub enum ChatMessageType {
  Chat = 0,
  System = 1,
  Info = 2,
}
