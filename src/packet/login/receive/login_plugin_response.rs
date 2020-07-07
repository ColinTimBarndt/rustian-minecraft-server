use crate::packet::{data::read, PacketParsingError, PacketSerialIn};

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
  fn read(buffer: &mut &[u8]) -> Result<LoginPluginResponse, PacketParsingError> {
    let m_id = read::var_u32(buffer)?;
    let successful = read::bool(buffer)?;
    Ok(Self {
      message_identifier: m_id,
      successful: successful,
      data: if successful {
        Some(read::byte_vec(buffer, buffer.len())?)
      } else {
        None
      },
    })
  }
}
