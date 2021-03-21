use crate::packet::{data::write, packet_ids::PLAY_CB_DECLARE_RECIPES, PacketSerialOut};
use crate::server::universe::crafting::recipe::{Recipe, ShapelessCraftingRecipe};
use std::default::Default;

/// # Declare Recipes
/// [Documentation](https://wiki.vg/Protocol#Declare_Recipes)
// TODO: impl Debug
pub struct DeclareRecipes<'a> {
  pub crafting_shapeless: Box<[&'a ShapelessCraftingRecipe]>, // etc. for every recipe type
}

impl PacketSerialOut for DeclareRecipes<'_> {
  const ID: u32 = PLAY_CB_DECLARE_RECIPES;
  fn write(&self, buffer: &mut Vec<u8>) -> Result<(), String> {
    let count = self.crafting_shapeless.len() /* + ... */;
    write::var_usize(buffer, count);
    for recipe in &*self.crafting_shapeless {
      (*recipe).serialize_recipe(buffer);
    }
    Ok(())
  }
}

impl Default for DeclareRecipes<'_> {
  fn default() -> Self {
    Self {
      crafting_shapeless: Box::new([]),
    }
  }
}
