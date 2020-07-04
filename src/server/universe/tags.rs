use crate::helpers::{NamespacedKey, Registry};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug)]
pub struct TagList<T: Registry> {
  tags: HashMap<NamespacedKey, Tag<T>>,
}

#[derive(Clone, Debug)]
pub struct Tag<T: Registry> {
  pub identifier: NamespacedKey,
  pub entries: HashSet<T>,
}

impl<T: Registry> TagList<T> {
  pub fn new() -> Self {
    Self {
      tags: HashMap::new(),
    }
  }

  pub fn len(&self) -> usize {
    self.tags.len()
  }

  pub fn iter(&self) -> std::collections::hash_map::Iter<NamespacedKey, Tag<T>> {
    self.tags.iter()
  }

  pub fn iter_mut(&mut self) -> std::collections::hash_map::IterMut<NamespacedKey, Tag<T>> {
    self.tags.iter_mut()
  }

  pub fn clear(&mut self) {
    self.tags.clear();
  }

  pub fn add_tag(&mut self, tag: Tag<T>) -> Option<Tag<T>> {
    self.tags.insert(tag.identifier.clone(), tag)
  }

  pub fn remove_tag(&mut self, identifier: &NamespacedKey) -> Option<Tag<T>> {
    self.tags.remove(identifier)
  }

  pub fn contains_tag(&self, identifier: &NamespacedKey) -> bool {
    self.tags.contains_key(identifier)
  }

  pub fn get_tag(&self, identifier: &NamespacedKey) -> Option<&Tag<T>> {
    self.tags.get(identifier)
  }

  pub fn get_tag_mut(&mut self, identifier: &NamespacedKey) -> Option<&mut Tag<T>> {
    self.tags.get_mut(identifier)
  }

  pub fn get_tags_of(&self, entry: T) -> HashSet<&NamespacedKey> {
    let mut keys = HashSet::with_capacity(4);
    for tag in self.tags.values() {
      if tag.entries.contains(&entry) {
        keys.insert(&tag.identifier);
      }
    }
    keys
  }

  pub fn has_tag(&self, entry: T, tag: &NamespacedKey) -> bool {
    if let Some(tag) = self.tags.get(tag) {
      if tag.entries.contains(&entry) {
        return true;
      }
    }
    false
  }
}
