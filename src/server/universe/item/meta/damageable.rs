extern crate nbt;
use nbt::Value;

#[derive(Clone, Copy, Debug)]
pub struct DamageableItemMeta {
  pub damage: Option<i32>,
  pub unbreakable: Option<bool>,
}

impl super::ItemMeta for DamageableItemMeta {
  fn apply_meta(&self, tag: &mut nbt::Blob) {
    if let Some(damage) = &self.damage {
      tag.insert("Damage", Value::Int(*damage));
    }
    if let Some(unbreakable) = &self.unbreakable {
      tag.insert("Unbreakable", Value::Byte(*unbreakable as i8));
    }
  }
}
