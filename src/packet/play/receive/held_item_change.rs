use super::super::send;
use crate::packet::{data::read, PacketSerialIn};
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
  fn consume_read(mut buffer: Vec<u8>) -> Result<Self, Box<dyn std::error::Error>> {
    Ok(Self {
      hotbar_slot: read::u16(&mut buffer)?.try_into()?,
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
