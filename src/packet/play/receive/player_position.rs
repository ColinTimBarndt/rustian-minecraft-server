use crate::helpers::Vec3d;
use crate::packet::{
  data::{finite_f64, read},
  packet_ids::PLAY_SB_PLAYER_POSITION,
  PacketParsingError, PacketSerialIn,
};

/// # Player Position
/// [Documentation](https://wiki.vg/Protocol#Player_Position)
///
/// Updates the player's XYZ position on the server.
///
/// Checking for moving too fast is achieved like this:
///
/// - Each server tick, the player's current position is stored
/// - When a player moves, the changes in x, y, and z coordinates
/// are compared with the positions from the previous tick (Δx, Δy, Δz)
/// - Total movement distance squared is computed as Δx² + Δy² + Δz²
/// - The expected movement distance squared is computed as
/// velocityX² + veloctyY² + velocityZ²
/// - If the total movement distance squared value minus the expected
/// movement distance squared value is more than 100 (300 if the player
/// is using an elytra), they are moving too fast.
///
/// If the player is moving too fast, it will be logged that "<player>
/// moved too quickly! " followed by the change in x, y, and z, and the
/// player will be teleported back to their current (before this packet)
/// serverside position.
///
/// Also, if the absolute value of X or the absolute value of Z is a
/// value greater than 3.2×107, or X, Y, or Z are not finite (either
/// positive infinity, negative infinity, or NaN), the client will be
/// kicked for “Invalid move player packet received”.
#[derive(Debug, Clone, Copy)]
pub struct PlayerPosition {
  pub position: Vec3d<f64>,
  pub on_ground: bool,
}

impl PacketSerialIn for PlayerPosition {
  const ID: u32 = PLAY_SB_PLAYER_POSITION;

  fn read(buffer: &mut &[u8]) -> Result<Self, PacketParsingError> {
    Ok(Self {
      position: Vec3d::new(
        finite_f64(read::f64(buffer)?)?,
        finite_f64(read::f64(buffer)?)?,
        finite_f64(read::f64(buffer)?)?,
      ),
      on_ground: read::bool(buffer)?,
    })
  }
}

impl AsRef<Vec3d<f64>> for PlayerPosition {
  fn as_ref(&self) -> &Vec3d<f64> {
    &self.position
  }
}
