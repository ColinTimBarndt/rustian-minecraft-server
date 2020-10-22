use crate::helpers::chat_components::ChatComponent;
use crate::packet::{data::write, packet_ids::PLAY_CB_CHAT_MESSAGE, PacketSerialOut};

/// # Chat Message (clientbound)
/// [Documentation](https://wiki.vg/Protocol#Chat_Message_.28clientbound.29)
///
/// Identifying the difference between Chat/System Message is important as it
/// helps respect the user's chat visibility options. See
/// [processing chat](https://wiki.vg/Chat#Processing_chat) for more info about
/// these positions.
#[derive(Clone, Debug)]
pub struct ChatMessage<'a> {
  pub message: &'a [ChatComponent],
  pub message_type: ChatMessageType,
}

impl<'a: 'b, 'b> ChatMessage<'b> {
  pub fn from_component(msg: &'a ChatComponent, msg_type: ChatMessageType) -> Self {
    Self {
      message: std::slice::from_ref(msg),
      message_type: msg_type,
    }
  }
  pub fn from_components(msg: &'a [ChatComponent], msg_type: ChatMessageType) -> Self {
    Self {
      message: msg,
      message_type: msg_type,
    }
  }
}

impl PacketSerialOut for ChatMessage<'_> {
  const ID: u32 = PLAY_CB_CHAT_MESSAGE;
  fn write(&self, buffer: &mut Vec<u8>) -> Result<(), String> {
    write::chat_components(buffer, self.message);
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
