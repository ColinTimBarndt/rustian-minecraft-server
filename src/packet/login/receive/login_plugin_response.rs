use crate::packet::{data::read, PacketSerialIn};
use std::error::Error;

#[derive(Debug)]
/// # Login Plugin Response
/// [Documentation](https://wiki.vg/Protocol#Login_Plugin_Response)
pub struct LoginPluginResponse {
  /// Should match ID from server.
  pub message_identifier: u32,
  /// `true` if the client understands the request, `false` otherwise. When `false`, no payload follows.
  pub successful: bool,
  /// Any data, depending on the channel. The length of this array must be inferred from the packet length.
  pub data: Option<Vec<u8>>,
}

impl PacketSerialIn for LoginPluginResponse {
  const ID: u32 = 0x02;
  fn consume_read(mut buffer: Vec<u8>) -> Result<LoginPluginResponse, Box<dyn Error>> {
    let m_id = read::var_u32(&mut buffer)?;
    let successful = read::bool(&mut buffer)?;
    Ok(Self {
      message_identifier: m_id,
      successful: successful,
      data: if successful {
        Some(buffer.drain(0..buffer.len()).collect())
      } else {
        None
      },
    })
  }
}
