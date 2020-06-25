use crate::helpers::{chat_components::ChatComponent, Color};
use crate::server::registries::Enchantment;
use std::collections::HashMap;

mod damageable;
pub use damageable::*;

mod block;
pub use block::*;

pub trait ItemMeta {
  fn apply_meta(&self, tag: &mut nbt::Blob);
}

#[derive(Clone, Debug)]
/// Represents the item "tag" property
///
/// _see [Item Structure](https://minecraft.gamepedia.com/Player.dat_format#Item_structure) on the Minecraft Wiki_
pub struct BasicItemMeta {
  pub display: Option<ItemDisplayMeta>,
  pub enchantments: Option<HashMap<Enchantment, i32>>,
  pub hide_flags: Option<HideFlags>, // TODO: attribute_modifiers
}

impl ItemMeta for BasicItemMeta {
  fn apply_meta(&self, tag: &mut nbt::Blob) {
    if let Some(display) = &self.display {
      let mut exists = false;
      let mut map = HashMap::new();
      if let Some(name) = &display.name {
        exists = true;
        // No error can be thrown here according to the doc
        map
          .insert("Name".into(), nbt::Value::String(name.make_json().dump()))
          .unwrap();
      }
      if exists {
        // No error can be thrown here according to the doc
        tag.insert("display", nbt::Value::Compound(map)).unwrap();
      }
    }
    // TODO: Enchantments
    if let Some(hide_flags) = &self.hide_flags {
      tag
        .insert("HideFlags", nbt::Value::Int(hide_flags.inner() as i32))
        // No error can be thrown here according to the doc
        .unwrap();
    }
  }
}

#[derive(Clone, Debug)]
/// Represents the [item tag "display" property](https://minecraft.gamepedia.com/Player.dat_format#Display_Properties)
pub struct ItemDisplayMeta {
  /// Custom name of the item
  pub name: Option<ChatComponent>,
  /// Additional text which is displayed below the item name
  pub lore: Option<Vec<ChatComponent>>,
  /// Changes the color of the item if it can be colored
  pub color: Option<Color>,
}

type HideFlagsType = u8;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
/// Wrapper for a bit field determining which parts of the item tooltip to hide
///
/// _See the [item tag HideFlags](https://minecraft.gamepedia.com/Player.dat_format#Display_Properties)_
pub struct HideFlags(pub HideFlagsType);

impl HideFlags {
  pub const ENCHANTMENTS: HideFlagsType = 0b000001;
  pub const ATTRIBUTE_MODIFIERS: HideFlagsType = 0b000010;
  pub const UNBREAKABLE: HideFlagsType = 0b000100;
  pub const CAN_DESTROY: HideFlagsType = 0b001000;
  pub const CAN_PLACE_ON: HideFlagsType = 0b010000;
  pub const OTHER: HideFlagsType = 0b100000;

  pub fn inner(&self) -> HideFlagsType {
    self.0
  }
  pub fn set(&mut self, flags: HideFlagsType) {
    self.0 = flags;
  }

  pub fn hides_enchantments(&self) -> bool {
    self.0 & HideFlags::ENCHANTMENTS > 0
  }
  pub fn hide_enchantments(&mut self, v: bool) {
    self.0 = (self.0 & !HideFlags::ENCHANTMENTS) | ((v as HideFlagsType) * HideFlags::ENCHANTMENTS);
  }

  pub fn hides_attribute_modifiers(&self) -> bool {
    self.0 & HideFlags::ATTRIBUTE_MODIFIERS > 0
  }
  pub fn hide_attribute_modifiers(&mut self, v: bool) {
    self.0 = (self.0 & !HideFlags::ATTRIBUTE_MODIFIERS)
      | ((v as HideFlagsType) * HideFlags::ATTRIBUTE_MODIFIERS);
  }

  pub fn hides_unbreakable(&self) -> bool {
    self.0 & HideFlags::UNBREAKABLE > 0
  }
  pub fn hide_unbreakable(&mut self, v: bool) {
    self.0 = (self.0 & !HideFlags::UNBREAKABLE) | ((v as HideFlagsType) * HideFlags::UNBREAKABLE);
  }

  pub fn hides_can_destroy(&self) -> bool {
    self.0 & HideFlags::CAN_DESTROY > 0
  }
  pub fn hide_can_destroy(&mut self, v: bool) {
    self.0 = (self.0 & !HideFlags::CAN_DESTROY) | ((v as HideFlagsType) * HideFlags::CAN_DESTROY);
  }

  pub fn hides_can_place_on(&self) -> bool {
    self.0 & HideFlags::CAN_PLACE_ON > 0
  }
  pub fn hide_can_place_on(&mut self, v: bool) {
    self.0 = (self.0 & !HideFlags::CAN_PLACE_ON) | ((v as HideFlagsType) * HideFlags::CAN_PLACE_ON);
  }

  pub fn hides_other(&self) -> bool {
    self.0 & HideFlags::OTHER > 0
  }
  pub fn hide_other(&mut self, v: bool) {
    self.0 = (self.0 & !HideFlags::OTHER) | ((v as HideFlagsType) * HideFlags::OTHER);
  }
}

impl std::ops::Deref for HideFlags {
  type Target = HideFlagsType;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl std::fmt::Display for HideFlags {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "Ench: {}, Mods: {}, Unbr: {}, Destr: {}, Place: {}, Other: {}",
      self.hides_enchantments(),
      self.hides_attribute_modifiers(),
      self.hides_unbreakable(),
      self.hides_can_destroy(),
      self.hides_can_place_on(),
      self.hides_other()
    )
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_hide_flags() {
    let mut flags = HideFlags(0);
    flags.hide_enchantments(true);
    flags.hide_attribute_modifiers(true);
    flags.hide_unbreakable(true);
    flags.hide_can_destroy(true);
    flags.hide_can_place_on(true);
    flags.hide_other(true);
    assert_eq!(flags.inner(), 0b111111);
    assert_eq!(flags.hides_enchantments(), true);
    assert_eq!(flags.hides_attribute_modifiers(), true);
    assert_eq!(flags.hides_unbreakable(), true);
    assert_eq!(flags.hides_can_destroy(), true);
    assert_eq!(flags.hides_can_place_on(), true);
    assert_eq!(flags.hides_other(), true);
  }
}
