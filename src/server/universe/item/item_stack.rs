use crate::server::registries::Item;
use std::default::Default;
extern crate nbt;
use super::meta::*;

#[derive(Clone, Debug)]
/// Implements the [Slot Data](https://wiki.vg/Slot_Data) structure
///
/// An ItemStack represents the content of one slot, which can either
/// be empty or have `1..=64` items of the same type with optional
/// NBT data
///
/// The `item`  is never `Air`. If `Air` is used as an argument, then the
/// `ItemData` will be set to `None`
pub struct ItemStack {
  pub data: Option<ItemData>,
}

#[derive(Clone, Debug)]
pub struct ItemData {
  pub item: Item,
  pub count: u8,
  pub meta: Option<BasicItemMeta>,
  pub damageable_meta: Option<DamageableItemMeta>,
  pub block_meta: Option<BlockItemMeta>,
}

impl ItemStack {
  pub fn new() -> Self {
    Self { data: None }
  }
  pub fn of_item(item: Item) -> Self {
    if item == Item::Air {
      Self::new()
    } else {
      Self {
        data: Some(ItemData {
          item,
          ..Default::default()
        }),
      }
    }
  }
  /// Note: `count` must be between 0 and 64 (inclusive)
  ///
  /// If `count` is 0 or the item is Air, the stack is empty
  pub fn of_items(item: Item, count: u8) -> Self {
    if count == 0 || item == Item::Air {
      return Self::new();
    }
    assert!(
      (1..=64).contains(&count),
      "count {} is not between 1 and 64 (inclusive)",
      count
    );
    Self {
      data: Some(ItemData {
        item,
        count,
        ..Default::default()
      }),
    }
  }
  /// Returns the item. If the ItemStack is empty, `Item::Air` is returned
  pub fn get_item(&self) -> Item {
    if let Some(data) = &self.data {
      data.item
    } else {
      Item::Air
    }
  }
  pub fn set_item(&mut self, item: Item) {
    if item == Item::Air {
      self.data = None;
    } else {
      if let Some(data) = &mut self.data {
        data.item = item;
      } else {
        self.data = Some(ItemData {
          item,
          ..Default::default()
        });
      }
    }
    // TODO: Set Meta that this item can't have to None
  }
  /// Returns 0 if the stack is empty
  pub fn get_count(&self) -> u8 {
    if let Some(data) = &self.data {
      data.count
    } else {
      0
    }
  }
  /// Count must be between 0 and 64 (inclusive)
  ///
  /// If `count` is 0 or the item is Air, the stack will be empty
  pub fn set_count(&mut self, count: u8) {
    if count == 0 {
      self.data = None;
    } else {
      assert!(
        (1..=64).contains(&count),
        "count {} is not between 1 and 64 (inclusive)",
        count
      );
      if let Some(data) = &mut self.data {
        data.count = count;
      }
    }
  }

  pub fn serialize_item(&self, buffer: &mut Vec<u8>) {
    use crate::packet::data::write;
    // https://wiki.vg/Slot_Data
    if let Some(data) = &self.data {
      write::bool(buffer, true); // Present
      write::var_usize(buffer, data.item as usize); // Item ID
      write::u8(buffer, data.count); // Item Count
      data.serialize_item_meta(buffer); // NBT
    } else {
      write::bool(buffer, false); // Present
    }
  }
}

impl ItemData {
  pub fn has_meta(&self) -> bool {
    self.meta.is_some() || self.block_meta.is_some()
  }
  pub fn serialize_item_meta(&self, buffer: &mut Vec<u8>) {
    // https://wiki.vg/Slot_Data
    if self.has_meta() {
      let mut blob = nbt::Blob::new();
      if let Some(meta) = &self.meta {
        meta.apply_meta(&mut blob);
      }
      if let Some(meta) = &self.damageable_meta {
        meta.apply_meta(&mut blob);
      }
      if let Some(meta) = &self.block_meta {
        meta.apply_meta(&mut blob);
      }
      blob.to_writer(buffer);
    } else {
      buffer.push(0);
    }
  }
}

impl Default for ItemStack {
  fn default() -> Self {
    Self::new()
  }
}

impl Default for ItemData {
  fn default() -> Self {
    Self {
      item: Item::Stone,
      count: 1,
      meta: None,
      damageable_meta: None,
      block_meta: None,
    }
  }
}
