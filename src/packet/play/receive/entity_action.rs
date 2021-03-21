use crate::packet::{
  data::read, packet_ids::PLAY_SB_ENTITY_ACTION, PacketParsingError, PacketSerialIn,
};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::convert::TryInto;

/// # Spawn Position
/// [Documentation](https://wiki.vg/Protocol#Entity_Action)
///
/// Sent by the client to indicate that it has performed
/// certain actions: sneaking (crouching), sprinting, exiting
/// a bed, jumping with a horse, and opening a horse's
/// inventory while riding it.
#[derive(Debug, Clone, Copy)]
pub struct EntityAction {
  pub entity: u32,
  pub action: EntityActionType,
}

impl PacketSerialIn for EntityAction {
  const ID: u32 = PLAY_SB_ENTITY_ACTION;
  fn read(buffer: &mut &[u8]) -> Result<Self, PacketParsingError> {
    let eid = read::var_u32(buffer)?;
    let r_action: EntityActionTypeRaw = FromPrimitive::from_u32(read::var_u32(buffer)?).ok_or(
      PacketParsingError::InvalidPacket("Invalid entity action type".into()),
    )?;
    let attr = read::var_u32(buffer)?;
    let action = match r_action {
      EntityActionTypeRaw::StartSneaking => EntityActionType::StartSneaking,
      EntityActionTypeRaw::StopSneaking => EntityActionType::StopSneaking,
      EntityActionTypeRaw::LeaveBed => EntityActionType::LeaveBed,
      EntityActionTypeRaw::StartSprinting => EntityActionType::StartSprinting,
      EntityActionTypeRaw::StopSprinting => EntityActionType::StopSprinting,
      EntityActionTypeRaw::StartJumpWithHorse => EntityActionType::StartJumpWithHorse(
        attr
          .try_into()
          .map_err(|_| PacketParsingError::InvalidPacket("Invalid action attribute".into()))?,
      ),
      EntityActionTypeRaw::StopJumpWithHorse => EntityActionType::StopJumpWithHorse,
      EntityActionTypeRaw::OpenHorseInventory => EntityActionType::OpenHorseInventory,
      EntityActionTypeRaw::StartFlyingWithElytra => EntityActionType::StartFlyingWithElytra,
    };
    Ok(Self {
      entity: eid,
      action,
    })
  }
}

#[derive(Debug, FromPrimitive, Clone, Copy)]
enum EntityActionTypeRaw {
  StartSneaking = 0,
  StopSneaking = 1,
  LeaveBed = 2,
  StartSprinting = 3,
  StopSprinting = 4,
  StartJumpWithHorse = 5,
  StopJumpWithHorse = 6,
  OpenHorseInventory = 7,
  StartFlyingWithElytra = 8,
}

#[derive(Debug, Clone, Copy)]
pub enum EntityActionType {
  StartSneaking,
  StopSneaking,
  LeaveBed,
  StartSprinting,
  StopSprinting,
  StartJumpWithHorse(u8),
  StopJumpWithHorse,
  OpenHorseInventory,
  StartFlyingWithElytra,
}
