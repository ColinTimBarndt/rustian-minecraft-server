use super::super::receive;
use crate::packet::{data::write, PacketSerialOut};

/// # Held Item Change (clientbound)
/// [Documentation](https://wiki.vg/Protocol#Held_Item_Change_.28clientbound.29)
///
/// Sent to change the player's slot selection.
#[derive(Clone, Debug)]
pub struct HeldItemChange {
  pub hotbar_slot: u8,
}

impl PacketSerialOut for HeldItemChange {
  const ID: u32 = 0x40;
  fn write(&self, buffer: &mut Vec<u8>) -> Result<(), String> {
    write::u8(buffer, self.hotbar_slot);
    Ok(())
  }
}

impl From<receive::HeldItemChange> for HeldItemChange {
  fn from(other: receive::HeldItemChange) -> Self {
    Self {
      hotbar_slot: other.hotbar_slot,
    }
  }
}
