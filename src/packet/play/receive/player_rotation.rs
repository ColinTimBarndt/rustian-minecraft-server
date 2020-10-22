use crate::helpers::{EulerAngle, Vec3d};
use crate::packet::{
  data::{finite_f32, read},
  packet_ids::PLAY_SB_PLAYER_ROTATION,
  PacketParsingError, PacketSerialIn,
};

/// # Player Rotation
/// [Documentation](https://wiki.vg/Protocol#Player_Rotation)
///
/// Updates the direction the player is looking in.
///
/// Yaw is measured in degrees, and does not follow classical
/// trigonometry rules. The unit circle of yaw on the XZ-plane
/// starts at (0, 1) and turns counterclockwise, with 90 at
/// (-1, 0), 180 at (0,-1) and 270 at (1, 0). Additionally,
/// yaw is not clamped to between 0 and 360 degrees; any number
/// is valid, including negative numbers and numbers greater
/// than 360.
///
/// Pitch is measured in degrees, where 0 is looking straight
/// ahead, -90 is looking straight up, and 90 is looking
/// straight down.
#[derive(Debug, Clone, Copy)]
pub struct PlayerRotation {
  pub rotation: EulerAngle,
  pub on_ground: bool,
}

impl PacketSerialIn for PlayerRotation {
  const ID: u32 = PLAY_SB_PLAYER_ROTATION;

  fn read(buffer: &mut &[u8]) -> Result<Self, PacketParsingError> {
    Ok(Self {
      rotation: EulerAngle::new(
        finite_f32(read::f32(buffer)?)?,
        finite_f32(read::f32(buffer)?)?,
        0.0,
      ),
      on_ground: read::bool(buffer)?,
    })
  }
}

impl AsRef<EulerAngle> for PlayerRotation {
  fn as_ref(&self) -> &EulerAngle {
    &self.rotation
  }
}
