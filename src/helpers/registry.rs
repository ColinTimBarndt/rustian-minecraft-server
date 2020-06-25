use crate::helpers::NamespacedKey;

pub const MINECRAFT_NAMESPACE: &'static str = &"minecraft";

pub trait Registry: Sized + Copy + Eq + std::hash::Hash {
  fn get_registry_key(&self) -> NamespacedKey;
  fn from_registry_key(key: &NamespacedKey) -> Option<Self>;
  fn get_id(&self) -> usize;
  fn from_id(id: usize) -> Option<Self>;
}
