use super::super::send;
use crate::packet::{data::read, PacketParsingError, PacketSerialIn};
use std::convert::TryInto;

/// # Held Item Change (serverbound)
/// [Documentation](https://wiki.vg/Protocol#Held_Item_Change_.28serverbound.29)
///
/// Sent when the player changes the slot selection
#[derive(Clone, Debug)]
pub struct HeldItemChange {
  pub hotbar_slot: u8,
}

impl PacketSerialIn for HeldItemChange {
  const ID: u32 = 0x23;
  fn read(buffer: &mut &[u8]) -> Result<Self, PacketParsingError> {
    Ok(Self {
      hotbar_slot: match read::u16(buffer)?.try_into() {
        Ok(slot) if slot <= 8 => slot,
        Ok(slot) => {
          return Err(PacketParsingError::InvalidPacket(format!(
            "Invalid hotbar slot: {}",
            slot
          )))
        }
        Err(e) => return Err(PacketParsingError::InvalidPacket(e.to_string())),
      },
    })
  }
}

impl From<send::HeldItemChange> for HeldItemChange {
  fn from(other: send::HeldItemChange) -> Self {
    Self {
      hotbar_slot: other.hotbar_slot,
    }
  }
}
