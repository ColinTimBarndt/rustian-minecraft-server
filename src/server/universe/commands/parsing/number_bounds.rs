use crate::packet::data::write;
use std::ops::{Bound, RangeBounds};

#[derive(Clone, Copy, Debug)]
pub struct NumberBounds<T: PartialOrd + Copy> {
  pub min: Option<T>,
  pub max: Option<T>,
}

macro_rules! impl_primitive {
  ($T:ident , $step:expr) => {
    impl NumberBounds<$T> {
      pub fn serialize_bounds(&self, buffer: &mut Vec<u8>) {
        write::u8(
          buffer,
          self.min.is_some() as u8 | ((self.max.is_some() as u8) << 1),
        );
        if let Some(min) = &self.min {
          write::$T(buffer, *min);
        }
        if let Some(max) = &self.max {
          write::$T(buffer, *max);
        }
      }
    }
    impl<B: RangeBounds<$T>> From<B> for NumberBounds<$T> {
      fn from(range: B) -> Self {
        Self {
          min: match range.start_bound() {
            Bound::Unbounded => None,
            Bound::Included(x) => Some(*x),
            Bound::Excluded(x) => Some(*x + $step),
          },
          max: match range.start_bound() {
            Bound::Unbounded => None,
            Bound::Included(x) => Some(*x),
            Bound::Excluded(x) => Some(*x - $step),
          },
        }
      }
    }
  };
}

impl_primitive!(f64, 0.0000000000001);
impl_primitive!(f32, 0.0000000000001);
impl_primitive!(i32, 1);

impl std::hash::Hash for NumberBounds<f64> {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    if let Some(min) = &self.min {
      state.write(&(*min).to_ne_bytes());
    }
    if let Some(max) = &self.max {
      state.write(&(*max).to_ne_bytes());
    }
  }
}

impl std::hash::Hash for NumberBounds<f32> {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    if let Some(min) = &self.min {
      state.write(&(*min).to_ne_bytes());
    }
    if let Some(max) = &self.max {
      state.write(&(*max).to_ne_bytes());
    }
  }
}

impl std::hash::Hash for NumberBounds<i32> {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    if let Some(min) = &self.min {
      min.hash(state);
    }
    if let Some(max) = &self.max {
      max.hash(state);
    }
  }
}
