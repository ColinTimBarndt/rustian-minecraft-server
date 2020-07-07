use crate::packet::{data::read, PacketParsingError, PacketSerialIn};

#[derive(Debug)]
/// # Encryption Response
/// [Documentation](https://wiki.vg/Protocol#Encryption_Response)
///
/// See [Protocol Encryption](https://wiki.vg/Protocol_Encryption) for details.
pub struct EncryptionResponse {
  pub shared_secret: Vec<u8>,
  pub verify_token: Vec<u8>,
}

impl PacketSerialIn for EncryptionResponse {
  const ID: u32 = 0x01;
  fn read(buffer: &mut &[u8]) -> Result<EncryptionResponse, PacketParsingError> {
    let sl = read::var_u32(buffer)? as usize;
    let s_secret = read::byte_vec(buffer, sl)?;

    let tl = read::var_u32(buffer)? as usize;
    let v_token = read::byte_vec(buffer, tl)?;

    Ok(Self {
      shared_secret: s_secret,
      verify_token: v_token,
    })
  }
}
