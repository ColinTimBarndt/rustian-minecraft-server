use crate::server::registries::RecipeSerializer;
use crate::server::universe::item::ItemStack;
use crate::server::universe::SharedPlayer;
use std::collections::HashSet;

mod crafting_shapeless;
pub use crafting_shapeless::*;

pub trait Recipe {
  const RECIPE_SERIALIZER: RecipeSerializer;
  fn serialize_recipe(&self, buffer: &mut Vec<u8>);
}

pub trait CraftingRecipe: Recipe {
  // TODO: Implement pattern recognition
  fn is_recipe_for(grid: CraftingGrid, context: Option<SharedPlayer>) -> bool;
  fn get_result_for(grid: CraftingGrid, context: Option<SharedPlayer>) -> ItemStack;
}

#[derive(Clone, Debug)]
pub struct CraftingGrid([ItemStack; 9]);

impl CraftingGrid {
  pub fn new() -> Self {
    Self([
      ItemStack::new(),
      ItemStack::new(),
      ItemStack::new(),
      ItemStack::new(),
      ItemStack::new(),
      ItemStack::new(),
      ItemStack::new(),
      ItemStack::new(),
      ItemStack::new(),
    ])
  }
}

#[derive(Clone, Debug)]
pub struct Ingredient {
  pub items: HashSet<ItemStack>,
}

impl Ingredient {
  pub fn serialize_ingredient(&self, buffer: &mut Vec<u8>) {
    use crate::packet::data::write;
    write::var_usize(buffer, self.items.len());
    for item in &self.items {
      item.serialize_item(buffer);
    }
  }
}
