use super::vector::Vec3d;
use std::ops::*;

/// See [Wikipedia](https://en.wikipedia.org/wiki/Quaternions_and_spatial_rotation)
///
/// This quaternion is always normalized.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Quaternion {
  w: f32,
  x: f32,
  y: f32,
  z: f32,
}

/// See [Wikipedia](https://en.wikipedia.org/wiki/Euler_angles)
///
/// This euler angle is always normalized.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct EulerAngle {
  yaw: f32,
  pitch: f32,
  roll: f32,
}

// Quaternion

impl Quaternion {
  pub fn new(w: f32, x: f32, y: f32, z: f32) -> Self {
    assert!(w.is_finite());
    assert!(x.is_finite());
    assert!(y.is_finite());
    assert!(z.is_finite());
    Self {
      w: w,
      x: x,
      y: y,
      z: z,
    }
  }
  pub fn normalize(&mut self) {
    let len = self.len();
    *self = Self {
      w: self.w / len,
      x: self.x / len,
      y: self.y / len,
      z: self.z / len,
    };
  }
  pub fn len(&self) -> f32 {
    (self.w * self.w + self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
  }
  /// See https://en.wikipedia.org/wiki/Slerp
  pub fn slerp(mut self, mut other: Self, t: f32) -> Self {
    self.normalize();
    other.normalize();
    let mut dot = self.dot(other);
    if dot < 0.0f32 {
      self = -self;
      dot = -dot;
    }
    if dot > 0.9995 {
      let mut result = self + (t * (other - self));
      result.normalize();
      result
    } else {
      let theta_0 = dot.acos();
      let theta = theta_0 * t;
      let sin_theta = theta.sin();
      let sin_theta_0 = theta_0.sin();

      let s0 = theta.cos() - dot * sin_theta / sin_theta_0;
      let s1 = sin_theta / sin_theta_0;
      (s0 * self) + (s1 * other)
    }
  }
  pub fn dot(self, other: Self) -> f32 {
    self.w * other.w + self.x * other.x + self.y * other.y + self.z * other.z
  }
  /// See https://en.wikipedia.org/wiki/Conversion_between_quaternions_and_Euler_angles
  pub fn to_euler(self) -> EulerAngle {
    let q = self;

    // roll
    let sinr_cosp = 2.0 * (q.w * q.x + q.y * q.z);
    let cosr_cosp = 1.0 - 2.0 * (q.x * q.x + q.y * q.y);
    let roll = f32::atan2(sinr_cosp, cosr_cosp).to_degrees();

    // pitch
    let sinp = 2.0 * (q.w * q.y - q.z * q.x);
    let pitch = if sinp.abs() >= 1.0 {
      f32::copysign(90.0, sinp) // use 90 degrees if out of range
    } else {
      f32::asin(sinp).to_degrees()
    };

    // yaw
    let siny_cosp = 2.0 * (q.w * q.z + q.x * q.y);
    let cosy_cosp = 1.0 - 2.0 * (q.y * q.y + q.z * q.z);
    let yaw = f32::atan2(siny_cosp, cosy_cosp).to_degrees();

    EulerAngle { yaw, pitch, roll }
  }
}

impl Neg for Quaternion {
  type Output = Self;
  #[inline]
  fn neg(self) -> Self {
    Self {
      w: -self.w,
      x: -self.x,
      y: -self.y,
      z: -self.z,
    }
  }
}

impl Mul<f32> for Quaternion {
  type Output = Self;
  #[inline]
  fn mul(self, rhs: f32) -> Self {
    Self {
      w: self.w * rhs,
      x: self.x * rhs,
      y: self.y * rhs,
      z: self.z * rhs,
    }
  }
}

impl Mul<Quaternion> for f32 {
  type Output = Quaternion;
  #[inline]
  fn mul(self, rhs: Quaternion) -> Quaternion {
    rhs * self
  }
}

impl Add for Quaternion {
  type Output = Self;
  fn add(self, rhs: Self) -> Self {
    Self {
      w: self.w + rhs.w,
      x: self.x + rhs.x,
      y: self.y + rhs.y,
      z: self.z + rhs.z,
    }
  }
}

impl Sub for Quaternion {
  type Output = Self;
  fn sub(self, rhs: Self) -> Self {
    Self {
      w: self.w - rhs.w,
      x: self.x - rhs.x,
      y: self.y - rhs.y,
      z: self.z - rhs.z,
    }
  }
}

// EulerAngle

impl EulerAngle {
  pub fn new(yaw: f32, pitch: f32, roll: f32) -> Self {
    assert!(yaw.is_finite());
    assert!(pitch.is_finite());
    assert!(roll.is_finite());
    Self { yaw, pitch, roll }
  }
  pub fn normalize(&mut self) {
    *self = Self {
      yaw: Self::enforce_bounds(self.yaw),
      pitch: self.pitch.max(-90.0).min(90.0),
      roll: Self::enforce_bounds(self.roll),
    }
  }
  pub fn set_yaw(&mut self, yaw: f32) {
    assert!(yaw.is_finite());
    self.yaw = yaw;
  }
  pub fn set_pitch(&mut self, pitch: f32) {
    assert!(pitch.is_finite());
    self.pitch = pitch;
  }
  pub fn set_roll(&mut self, roll: f32) {
    assert!(roll.is_finite());
    self.roll = roll;
  }
  pub fn get_yaw(&self) -> f32 {
    self.yaw
  }
  pub fn get_pitch(&self) -> f32 {
    self.pitch
  }
  pub fn get_roll(&self) -> f32 {
    self.roll
  }
  /// See [https://en.wikipedia.org/wiki/Conversion_between_quaternions_and_Euler_angles]
  pub fn to_quaternion(self) -> Quaternion {
    let yaw = self.yaw.to_radians() * 0.5;
    let pitch = self.pitch.to_radians() * 0.5;
    let roll = self.roll.to_radians() * 0.5;

    let cy = f32::cos(yaw);
    let sy = f32::sin(yaw);
    let cp = f32::cos(pitch);
    let sp = f32::sin(pitch);
    let cr = f32::cos(roll);
    let sr = f32::sin(roll);

    Quaternion {
      w: cr * cp * cy + sr * sp * sy,
      x: sr * cp * cy - cr * sp * sy,
      y: cr * sp * cy + sr * cp * sy,
      z: cr * cp * sy - sr * sp * cy,
    }
  }
  pub fn to_unit_vec(&self) -> Vec3d<f64> {
    // https://github.com/Bukkit/Bukkit/blob/master/src/main/java/org/bukkit/Location.java#L244
    let rot_x = (self.yaw as f64).to_radians();
    let rot_y = (self.pitch as f64).to_radians();
    let xz = rot_y.cos();
    Vec3d::new(-xz * rot_x.sin(), -rot_y.sin(), xz * rot_x.cos())
  }
  /// Note: The vec **must be normalized** first.
  pub fn from_unit_vec(vec: Vec3d<f64>) -> EulerAngle {
    //debug_assert!(vec.is_normalized(), "vector is not a unit vector");
    // https://github.com/Bukkit/Bukkit/blob/master/src/main/java/org/bukkit/Location.java#L264
    let (x, y, z) = vec.into();
    if x == 0.0 && z == 0.0 {
      EulerAngle {
        pitch: if y > 0.0 { -90.0 } else { 90.0 },
        ..Default::default()
      }
    } else {
      use std::f64::consts::PI;
      let theta = f64::atan2(-x, z);
      let xz = (x * x + z * z).sqrt();
      EulerAngle {
        yaw: ((theta + 2.0 * PI) % (2.0 * PI)).to_degrees() as f32,
        pitch: (-y / xz).atan().to_degrees() as f32,
        ..Default::default()
      }
    }
  }
  pub fn from_vec(vec: Vec3d<f64>) -> EulerAngle {
    use crate::helpers::vector::Normalize;
    Self::from_unit_vec(vec.normalize())
  }
  fn enforce_bounds(ang: f32) -> f32 {
    ((ang % 360.0) + 360.0) % 360.0
  }
}

impl Add for EulerAngle {
  type Output = Self;
  fn add(self, other: Self) -> Self::Output {
    Self::new(
      self.yaw + other.yaw,
      self.pitch + other.pitch,
      self.roll + other.roll,
    )
  }
}

impl AddAssign for EulerAngle {
  fn add_assign(&mut self, other: Self) {
    let new = Self::new(
      self.yaw + other.yaw,
      self.pitch + other.pitch,
      self.roll + other.roll,
    );
    *self = new;
  }
}

impl Sub for EulerAngle {
  type Output = Self;
  fn sub(self, other: Self) -> Self::Output {
    Self::new(
      self.yaw - other.yaw,
      self.pitch - other.pitch,
      self.roll - other.roll,
    )
  }
}

impl SubAssign for EulerAngle {
  fn sub_assign(&mut self, other: Self) {
    let new = Self::new(
      self.yaw - other.yaw,
      self.pitch - other.pitch,
      self.roll - other.roll,
    );
    *self = new;
  }
}

#[test]
fn test_conversion_euler_to_quaternion() {
  use rand::prelude::*;
  let mut rng = rand::thread_rng();
  let mut total_precision: f64 = 0.0;
  let total_tests = 100000;
  for _ in 0..total_tests {
    let mut euler = EulerAngle::new(
      (rng.next_u32() % 36000000) as f32 / 100000.0,
      (rng.next_u32() % 18000000) as f32 / 100000.0 - 90.0,
      (rng.next_u32() % 36000000) as f32 / 100000.0,
    );
    let q = euler.to_quaternion();
    let mut euler2 = q.to_euler();
    euler.normalize();
    euler2.normalize();
    let d_yaw = (euler2.get_yaw() - euler.get_yaw()).abs();
    let d_pitch = (euler2.get_pitch() - euler.get_pitch()).abs();
    let d_roll = (euler2.get_roll() - euler.get_roll()).abs();
    // Some precision may be lost. This can vary a lot.
    // Therefore, only an average is calculated.
    // Sometimes no precision is lost at all and somatimes,
    // results have a difference greater than 1.0
    total_precision += (d_yaw + d_pitch + d_roll) as f64;
  }
  total_precision /= total_tests as f64;
  assert!(
    total_precision < 0.0001,
    "Average precision: {}",
    total_precision
  );
}

#[test]
fn test_euler_to_unit_vec() {
  assert_eq!(
    round_vec(EulerAngle::new(0.0, 0.0, 0.0).to_unit_vec()),
    Vec3d::new(0.0, 0.0, 1.0)
  );
  assert_eq!(
    round_vec(EulerAngle::new(0.0, 90.0, 0.0).to_unit_vec()),
    Vec3d::new(0.0, -1.0, 0.0)
  );
  assert_eq!(
    round_vec(EulerAngle::new(0.0, -90.0, 0.0).to_unit_vec()),
    Vec3d::new(0.0, 1.0, 0.0)
  );
  assert_eq!(
    round_vec(EulerAngle::new(90.0, 0.0, 0.0).to_unit_vec()),
    Vec3d::new(-1.0, 0.0, 0.0)
  );
  assert_eq!(
    round_vec(EulerAngle::new(-90.0, 0.0, 0.0).to_unit_vec()),
    Vec3d::new(1.0, 0.0, 0.0)
  );

  #[inline]
  fn round_vec(vec: Vec3d<f64>) -> Vec3d<f64> {
    Vec3d::new(round(vec.x, 16.0), round(vec.y, 16.0), round(vec.z, 16.0))
  }
  #[inline]
  fn round(n: f64, d: f64) -> f64 {
    (n * d).round() / d
  }
}
