#[derive(Clone, Copy)]
pub struct NibbleArray4096 {
  inner: [u8; 2048],
}

impl NibbleArray4096 {
  pub fn new() -> Self {
    Self { inner: [0; 2048] }
  }
  pub fn len() -> usize {
    4096
  }
  fn to_index(index: usize) -> (usize, bool) {
    assert!(index < 4096, "Index out of bounds");
    (index >> 1, (index & 1) != 0)
  }
  pub fn get(&self, index: usize) -> u8 {
    let (i, h) = Self::to_index(index);
    if h {
      self.inner[i] >> 4
    } else {
      self.inner[i] & 0b1111
    }
  }
  pub fn set(&mut self, index: usize, val: u8) {
    assert!(val < 16);
    let (i, h) = Self::to_index(index);
    if h {
      self.inner[i] &= (val << 4) | 0b1111;
    } else {
      self.inner[i] &= (0b1111 << 4) | val;
    }
  }
  pub fn set_at_pos(&mut self, x: u8, y: u8, z: u8, val: u8) {
    assert!(x < 16);
    assert!(y < 16);
    assert!(z < 16);
    self.set(((y as usize) << 8) | ((z as usize) << 4) | x as usize, val);
  }
}

impl From<[u8; 2048]> for NibbleArray4096 {
  fn from(f: [u8; 2048]) -> Self {
    Self { inner: f }
  }
}

impl AsRef<[u8; 2048]> for NibbleArray4096 {
  fn as_ref(&self) -> &[u8; 2048] {
    &self.inner
  }
}

impl IntoIterator for NibbleArray4096 {
  type Item = u8;
  type IntoIter = NibbleArray4096Iterator;
  fn into_iter(self) -> Self::IntoIter {
    Self::IntoIter {
      index: 0,
      array: self.inner,
    }
  }
}
impl<'a> IntoIterator for &'a NibbleArray4096 {
  type Item = u8;
  type IntoIter = NibbleArray4096RefIterator<'a>;
  fn into_iter(self) -> Self::IntoIter {
    Self::IntoIter {
      index: 0,
      array: &self.inner,
    }
  }
}

pub struct NibbleArray4096Iterator {
  index: usize,
  array: [u8; 2048],
}
impl Iterator for NibbleArray4096Iterator {
  type Item = u8;
  fn next(&mut self) -> Option<Self::Item> {
    let i = self.index >> 1;
    let h = self.index & 1 != 0;
    self.index += 1;
    match h {
      false => {
        if self.index >= 2048 {
          None
        } else {
          Some(self.array[i] & 0b1111)
        }
      }
      true => Some(self.array[i] >> 4),
    }
  }
}

pub struct NibbleArray4096RefIterator<'a> {
  index: usize,
  array: &'a [u8; 2048],
}
impl Iterator for NibbleArray4096RefIterator<'_> {
  type Item = u8;
  fn next(&mut self) -> Option<Self::Item> {
    let i = self.index >> 1;
    let h = self.index & 1 != 0;
    self.index += 1;
    match h {
      false => {
        if self.index >= 2048 {
          None
        } else {
          Some(self.array[i] & 0b1111)
        }
      }
      true => Some(self.array[i] >> 4),
    }
  }
}
