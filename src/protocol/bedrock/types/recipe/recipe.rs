use crate::protocol::bedrock::types::recipe::furnace_recipe::FurnaceRecipe;
use crate::protocol::bedrock::types::recipe::multi_recipe::MultiRecipe;
use crate::protocol::bedrock::types::recipe::shaped_recipe::ShapedRecipe;
use crate::protocol::bedrock::types::recipe::shapeless_recipe::ShapelessRecipe;
use crate::protocol::bedrock::types::recipe::smithing_transform_recipe::SmithingTransformRecipe;
use crate::protocol::bedrock::types::recipe::smithing_trim_recipe::SmithingTrimRecipe;
use binary_utils::binary::Stream;
use std::fmt::Debug;

#[derive(serde::Serialize, Debug)]
pub enum Recipe {
    Furnace(FurnaceRecipe),
    Shaped(ShapedRecipe),
    Shapeless(ShapelessRecipe),
    Multi(MultiRecipe),
    SmitingTransform(SmithingTransformRecipe),
    SmithingTrim(SmithingTrimRecipe),
}

impl Recipe {
    pub fn get_selected_type_id(&self) -> i32 {
        match self {
            Recipe::Furnace(r) => r.get_selected_type_id(),
            Recipe::Shaped(r) => r.get_selected_type_id(),
            Recipe::Shapeless(r) => r.get_selected_type_id(),
            Recipe::Multi(r) => r.get_selected_type_id(),
            Recipe::SmitingTransform(r) => r.get_selected_type_id(),
            Recipe::SmithingTrim(r) => r.get_selected_type_id(),
        }
    }

    pub fn write(&mut self, stream: &mut Stream) {
        match self {
            Recipe::Furnace(r) => r.write(stream),
            Recipe::Shaped(r) => r.write(stream),
            Recipe::Shapeless(r) => r.write(stream),
            Recipe::Multi(r) => r.write(stream),
            Recipe::SmitingTransform(r) => r.write(stream),
            Recipe::SmithingTrim(r) => r.write(stream),
        }
    }
}
