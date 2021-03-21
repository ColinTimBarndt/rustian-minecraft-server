use super::CraftingGrid;
use crate::packet::data::write;
use crate::server::registries::RecipeSerializer;
use crate::server::universe::item::ItemStack;

pub struct ShapelessCraftingRecipe {
    pub group: String,
    pub ingredients: Vec<super::Ingredient>,
    pub result: ItemStack,
}

impl super::Recipe for ShapelessCraftingRecipe {
    const RECIPE_SERIALIZER: RecipeSerializer = RecipeSerializer::CraftingShapeless;
    fn serialize_recipe(&self, buffer: &mut Vec<u8>) {
        write::string(buffer, &self.group.clone());
        write::var_usize(buffer, self.ingredients.len());
        for ingredient in &self.ingredients {
            ingredient.serialize_ingredient(buffer);
        }
        self.result.serialize_item(buffer);
    }
}

impl super::CraftingRecipe for ShapelessCraftingRecipe {
    fn is_recipe_for(_grid: CraftingGrid) -> bool {
        unimplemented!(); // TODO
    }
    fn get_result_for(_grid: CraftingGrid) -> ItemStack {
        unimplemented!(); // TODO
    }
}
