use crate::packet::{
  data::read, packet_ids::PLAY_SB_TELEPORT_CONFIRM, PacketParsingError, PacketSerialIn,
};

/// # Teleport Confirm
/// [Documentation](https://wiki.vg/Protocol#Teleport_Confirm)
///
/// Sent by client as confirmation of [Player Position And Look](https://wiki.vg/Protocol#Player_Position_And_Look_.28clientbound.29).
pub struct TeleportConfirm {
  pub id: u16,
}

impl PacketSerialIn for TeleportConfirm {
  const ID: u32 = PLAY_SB_TELEPORT_CONFIRM;

  fn read(buffer: &mut &[u8]) -> Result<Self, PacketParsingError> {
    use std::convert::TryInto;
    Ok(Self {
      id: read::var_u32(buffer)?
        .try_into()
        .map_err(|_| PacketParsingError::VarNumberTooBig)?,
    })
  }
}
