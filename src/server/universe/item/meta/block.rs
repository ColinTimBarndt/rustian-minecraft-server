use crate::helpers::Registry;
use crate::server::registries::Block;
extern crate nbt;
use nbt::Value;

#[derive(Clone, Debug)]
pub struct BlockItemMeta {
  pub can_place_on: Vec<Block>,
  // TODO: block_entity_tag
  // TODO: block_state_tag
}

impl super::ItemMeta for BlockItemMeta {
  fn apply_meta(&self, tag: &mut nbt::Blob) {
    if self.can_place_on.len() > 0 {
      tag
        .insert(
          "CanPlaceOn",
          Value::List(
            self
              .can_place_on
              .iter()
              .map(|block| Value::String(Registry::get_registry_key(block).to_string()))
              .collect(),
          ),
        )
        .unwrap();
    }
  }
}
