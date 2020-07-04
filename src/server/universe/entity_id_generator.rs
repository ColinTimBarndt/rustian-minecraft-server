use std::collections::VecDeque;

/// This ID generator is used to generate partially incrementing
/// entity Ids. The advantage of this generator is that the IDs
/// are kept as small as possible by re-using IDs that aren't
/// used any more.
pub struct EntityIdGenerator {
  ids: Vec<bool>,
  free_ids: VecDeque<u32>,
}

impl EntityIdGenerator {
  pub fn new() -> Self {
    Self {
      ids: Vec::with_capacity(50),
      free_ids: VecDeque::new(),
    }
  }
  /// Reserves a new entity ID that can't
  /// be used by other entities until it is freed
  pub fn reserve(&mut self) -> u32 {
    if let Some(id) = self.free_ids.pop_front() {
      self.ids[id as usize] = true;
      id
    } else {
      let id = self.ids.len() as u32;
      self.ids.push(true);
      id
    }
  }
  /// Allows other entities to reserve this ID
  pub fn free(&mut self, id: u32) {
    let us = id as usize;
    if us < self.ids.len() {
      let prev = self.ids[us];
      if !prev {
        return;
      }
      if us - 1 == self.ids.len() {
        self.ids.pop();
        // Try to further decrease both array sizes by simplifying the arrays
        while self.ids.len() > 0 && !self.ids[self.ids.len() - 1] {
          self.ids.pop().unwrap();
          let freed_id = self.ids.len() as u32;
          let idx = self.free_ids.iter().position(|x| *x == freed_id).unwrap();
          self.free_ids.remove(idx);
        }
      } else {
        self.ids[us] = false;
        self.free_ids.push_back(id);
      }
    }
  }
  pub fn clean_up(&mut self) {
    // //Unstable:
    // self.free.make_contiguous().sort();
    let mut temp: Vec<u32> = self.free_ids.iter().map(|x| *x).collect();
    temp.sort_unstable();
    self.free_ids = temp.into_iter().collect();
  }
}
