use crate::helpers::{EulerAngle, Vec3d};
use crate::packet::{
  data::{finite_f32, finite_f64, read},
  packet_ids::PLAY_SB_PLAYER_POSITION_AND_ROTATION,
  PacketParsingError, PacketSerialIn,
};

/// # Player Position And Rotation (serverbound)
/// [Documentation](https://wiki.vg/Protocol#Player_Position_And_Rotation_.28serverbound.29)
///
/// A combination of Player Rotation and Player Position.
#[derive(Debug, Clone, Copy)]
pub struct PlayerPositionAndRotation {
  pub position: Vec3d<f64>,
  pub rotation: EulerAngle,
  pub on_ground: bool,
}

impl PacketSerialIn for PlayerPositionAndRotation {
  const ID: u32 = PLAY_SB_PLAYER_POSITION_AND_ROTATION;

  fn read(buffer: &mut &[u8]) -> Result<Self, PacketParsingError> {
    Ok(Self {
      position: Vec3d::new(
        finite_f64(read::f64(buffer)?)?,
        finite_f64(read::f64(buffer)?)?,
        finite_f64(read::f64(buffer)?)?,
      ),
      rotation: EulerAngle::new(
        finite_f32(read::f32(buffer)?)?,
        finite_f32(read::f32(buffer)?)?,
        0.0,
      ),
      on_ground: read::bool(buffer)?,
    })
  }
}

impl AsRef<Vec3d<f64>> for PlayerPositionAndRotation {
  fn as_ref(&self) -> &Vec3d<f64> {
    &self.position
  }
}

impl AsRef<EulerAngle> for PlayerPositionAndRotation {
  fn as_ref(&self) -> &EulerAngle {
    &self.rotation
  }
}
