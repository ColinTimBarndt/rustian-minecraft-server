use crate::helpers::NamespacedKey;
use crate::packet::{data::write, packet_ids::PLAY_CB_UNLOCK_RECIPES, PacketSerialOut};

/// # Unlock Recipes
/// [Documentation](https://wiki.vg/Protocol#Unlock_Recipes)
#[derive(Debug)]
pub struct UnlockRecipes<'a> {
  pub action: UnlockRecipesAction<'a>,
  pub crafting_recipe_book: RecipeBookStatus,
  pub smelting_recipe_book: RecipeBookStatus,
}

impl PacketSerialOut for UnlockRecipes<'_> {
  const ID: u32 = PLAY_CB_UNLOCK_RECIPES;
  fn write(&self, buffer: &mut Vec<u8>) -> Result<(), String> {
    write::var_u8(buffer, self.action.get_id());
    self.crafting_recipe_book.serialize(buffer);
    self.smelting_recipe_book.serialize(buffer);
    match self.action {
      UnlockRecipesAction::Initialize {
        init_recipes: init,
        unlocked_recipes: unlocked,
      } => {
        NamespacedKey::serialize_vec(buffer, unlocked);
        NamespacedKey::serialize_vec(buffer, init);
      }
      UnlockRecipesAction::Add(add) => {
        NamespacedKey::serialize_vec(buffer, add);
      }
      UnlockRecipesAction::Remove(rem) => {
        NamespacedKey::serialize_vec(buffer, rem);
      }
    }
    Ok(())
  }
}

#[derive(Debug)]
pub enum UnlockRecipesAction<'a> {
  Initialize {
    init_recipes: &'a Vec<NamespacedKey>,
    unlocked_recipes: &'a Vec<NamespacedKey>,
  },
  Add(&'a Vec<NamespacedKey>),
  Remove(&'a Vec<NamespacedKey>),
}

impl UnlockRecipesAction<'_> {
  pub fn get_id(&self) -> u8 {
    match self {
      Self::Initialize {
        init_recipes: _,
        unlocked_recipes: _,
      } => 0,
      Self::Add(_) => 1,
      Self::Remove(_) => 2,
    }
  }
}

#[derive(Clone, Copy, Debug)]
pub struct RecipeBookStatus {
  open: bool,
  filter_active: bool,
}

impl RecipeBookStatus {
  fn serialize(&self, buffer: &mut Vec<u8>) {
    write::bool(buffer, self.open);
    write::bool(buffer, self.filter_active);
  }
}

impl Default for RecipeBookStatus {
  fn default() -> Self {
    Self {
      open: false,
      filter_active: false,
    }
  }
}
