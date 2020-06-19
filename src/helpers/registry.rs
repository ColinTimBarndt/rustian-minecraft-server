use crate::helpers::NamespacedKey;

pub const MINECRAFT_NAMESPACE: &'static str = &"minecraft";

pub trait Registry: Sized {
  fn get_registry_key(&self) -> NamespacedKey;
  fn from_registry_key(key: &NamespacedKey) -> Option<Self>;
}
