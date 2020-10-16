use crate::helpers::{EulerAngle, Vec3d};
use crate::packet::{data::write, PacketSerialOut};

const POSITION_THRESHOLD: f64 = 16.0;
const ROTATION_THRESHOLD: f32 = 30.0;

/// # Player Position And Look (clientbound)
/// [Documentation](https://wiki.vg/Protocol#Player_Position_And_Look_.28clientbound.29)
///
/// Updates the player's position on the server. This packet will
/// also close the “Downloading Terrain” screen when joining/respawning.
///
/// If the distance between the last known position of the player on the
/// server and the new position set by this packet is greater than 100
/// meters, the client will be kicked for “You moved too quickly :( (Hacking?)”.
///
/// Also if the fixed-point number of X or Z is set greater than 3.2E7D
/// the client will be kicked for “Illegal position”.
///
/// Yaw is measured in degrees, and does not follow classical trigonometry
/// rules. The unit circle of yaw on the XZ-plane starts at (0, 1) and
/// turns counterclockwise, with 90 at (-1, 0), 180 at (0, -1) and 270
/// at (1, 0). Additionally, yaw is not clamped to between 0 and 360
/// degrees; any number is valid, including negative numbers and numbers
/// greater than 360.
///
/// Pitch is measured in degrees, where 0 is looking straight ahead, -90
/// is looking straight up, and 90 is looking straight down.
#[derive(Debug, Clone, Copy)]
pub struct PlayerPositionAndLook {
  pub x: f64,
  pub y: f64,
  pub z: f64,
  pub yaw: f32,
  pub pitch: f32,
  pub flags: Flags,
  pub id: u16,
}

#[derive(Clone, Copy)]
pub struct PlayerPositionAndLookFlags(u8);
type Flags = PlayerPositionAndLookFlags;

impl PacketSerialOut for PlayerPositionAndLook {
  const ID: u32 = 0x36;
  fn write(&self, buffer: &mut Vec<u8>) -> Result<(), String> {
    write::f64(buffer, self.x);
    write::f64(buffer, self.y);
    write::f64(buffer, self.z);
    write::f32(buffer, self.yaw);
    write::f32(buffer, self.pitch);
    write::u8(buffer, *self.flags);
    write::var_u16(buffer, self.id);
    Ok(())
  }
  fn consume_write(self, buffer: &mut Vec<u8>) -> Result<(), String> {
    write::f64(buffer, self.x);
    write::f64(buffer, self.y);
    write::f64(buffer, self.z);
    write::f32(buffer, self.yaw);
    write::f32(buffer, self.pitch);
    write::u8(buffer, *self.flags);
    write::var_u16(buffer, self.id);
    Ok(())
  }
}

impl PlayerPositionAndLook {
  pub fn create(
    id: u16,
    pos_old: Vec3d<f64>,
    pos_new: Vec3d<f64>,
    rot_old: EulerAngle,
    rot_new: EulerAngle,
  ) -> Self {
    let d_pos = pos_new - pos_old;
    let d_rot = rot_new - rot_old;
    let mut flags = Flags::new();
    flags.set_x_relative(d_pos.x.abs() < POSITION_THRESHOLD);
    flags.set_y_relative(d_pos.y.abs() < POSITION_THRESHOLD);
    flags.set_z_relative(d_pos.z.abs() < POSITION_THRESHOLD);
    flags.set_yaw_relative(d_rot.get_yaw().abs() < ROTATION_THRESHOLD);
    flags.set_pitch_relative(d_rot.get_pitch().abs() < ROTATION_THRESHOLD);
    Self {
      id,
      x: if flags.is_x_relative() {
        d_pos.x
      } else {
        pos_new.x
      },
      y: if flags.is_y_relative() {
        d_pos.y
      } else {
        pos_new.y
      },
      z: if flags.is_z_relative() {
        d_pos.z
      } else {
        pos_new.z
      },
      yaw: if flags.is_yaw_relative() {
        d_rot.get_yaw()
      } else {
        rot_new.get_yaw()
      },
      pitch: if flags.is_pitch_relative() {
        d_rot.get_pitch()
      } else {
        rot_new.get_pitch()
      },
      flags,
    }
  }
  pub fn create_abs(id: u16, pos: Vec3d<f64>, rot: EulerAngle) -> Self {
    Self {
      id,
      x: pos.x,
      y: pos.y,
      z: pos.z,
      yaw: rot.get_yaw(),
      pitch: rot.get_pitch(),
      flags: Flags::new(),
    }
  }
}

impl Flags {
  const ALL: u8 = 0b00011111;
  const X: u8 = 0b00000001;
  const Y: u8 = 0b00000010;
  const Z: u8 = 0b00000100;
  const YAW: u8 = 0b00001000;
  const PITCH: u8 = 0b00010000;

  pub fn new() -> Self {
    Self(0)
  }

  pub fn is_x_relative(&self) -> bool {
    self.0 & Self::X != 0
  }
  pub fn is_y_relative(&self) -> bool {
    self.0 & Self::Y != 0
  }
  pub fn is_z_relative(&self) -> bool {
    self.0 & Self::Z != 0
  }
  pub fn is_yaw_relative(&self) -> bool {
    self.0 & Self::YAW != 0
  }
  pub fn is_pitch_relative(&self) -> bool {
    self.0 & Self::PITCH != 0
  }
  pub fn set_x_relative(&mut self, r: bool) {
    self.0 ^= (self.is_x_relative() ^ r) as u8 * Self::X;
  }
  pub fn set_y_relative(&mut self, r: bool) {
    self.0 ^= (self.is_y_relative() ^ r) as u8 * Self::Y;
  }
  pub fn set_z_relative(&mut self, r: bool) {
    self.0 ^= (self.is_z_relative() ^ r) as u8 * Self::Z;
  }
  pub fn set_yaw_relative(&mut self, r: bool) {
    self.0 ^= (self.is_yaw_relative() ^ r) as u8 * Self::YAW;
  }
  pub fn set_pitch_relative(&mut self, r: bool) {
    self.0 ^= (self.is_pitch_relative() ^ r) as u8 * Self::PITCH;
  }
}

impl From<u8> for Flags {
  fn from(byte: u8) -> Flags {
    Self(byte & Flags::ALL)
  }
}

impl From<Flags> for u8 {
  fn from(flags: Flags) -> u8 {
    flags.0
  }
}

impl std::ops::Deref for Flags {
  type Target = u8;
  fn deref(&self) -> &u8 {
    &self.0
  }
}

impl std::fmt::Debug for Flags {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Flags({:#b})", self.0)
  }
}
