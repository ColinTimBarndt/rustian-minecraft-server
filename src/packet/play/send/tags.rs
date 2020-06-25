use crate::helpers::Registry;
use crate::packet::{data::write, PacketSerialOut};
use crate::server::registries::{Block, EntityType, Fluid, Item};
use crate::server::universe::tags::{Tag, TagList};
// use std::string::ToString;

/// # Tags
/// [Documentation](https://wiki.vg/Protocol#Tags)
#[derive(Clone, Debug)]
pub struct Tags {
  pub blocks: TagList<Block>,
  pub items: TagList<Item>,
  pub fluids: TagList<Fluid>,
  pub entities: TagList<EntityType>,
}

impl PacketSerialOut for Tags {
  const ID: u32 = 0x5C;
  fn write(&self, buffer: &mut Vec<u8>) -> Result<(), String> {
    write_tag_list(buffer, &self.blocks);
    write_tag_list(buffer, &self.items);
    write_tag_list(buffer, &self.fluids);
    write_tag_list(buffer, &self.entities);
    Ok(())
  }
}

impl Tags {
  pub fn new() -> Self {
    Default::default()
  }
}

impl Default for Tags {
  fn default() -> Self {
    Self {
      blocks: TagList::new(),
      items: TagList::new(),
      fluids: TagList::new(),
      entities: TagList::new(),
    }
  }
}

fn write_tag_list<T: Registry>(buffer: &mut Vec<u8>, list: &TagList<T>) {
  write::var_usize(buffer, list.len());
  for (_, tag) in list.iter() {
    write_tag(buffer, tag);
  }
}

fn write_tag<T: Registry>(buffer: &mut Vec<u8>, tag: &Tag<T>) {
  write::string(buffer, tag.identifier.to_string()); // Identifier
  write::var_usize(buffer, tag.entries.len()); // Number of elements in the following array
  for entry in &tag.entries {
    write::var_usize(buffer, entry.get_id()); // Numeric ID of the entry
  }
}
