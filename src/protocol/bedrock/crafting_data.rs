use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::types::recipe::furnace_recipe::FurnaceRecipe;
use crate::protocol::bedrock::types::recipe::material_reducer_recipe::MaterialReducerRecipe;
use crate::protocol::bedrock::types::recipe::material_reducer_recipe_output::MaterialReducerRecipeOutput;
use crate::protocol::bedrock::types::recipe::multi_recipe::MultiRecipe;
use crate::protocol::bedrock::types::recipe::potion_container_change_recipe::PotionContainerChangeRecipe;
use crate::protocol::bedrock::types::recipe::potion_type_recipe::PotionTypeRecipe;
use crate::protocol::bedrock::types::recipe::recipe::Recipe;
use crate::protocol::bedrock::types::recipe::shaped_recipe::ShapedRecipe;
use crate::protocol::bedrock::types::recipe::shapeless_recipe::ShapelessRecipe;
use crate::protocol::bedrock::types::recipe::smithing_transform_recipe::SmithingTransformRecipe;
use crate::protocol::bedrock::types::recipe::smithing_trim_recipe::SmithingTrimRecipe;
use binary_utils::binary::Stream;
use std::any::Any;

#[derive(serde::Serialize, Debug)]
pub struct CraftingData {
    pub recipes: Vec<Recipe>,
    pub potion_type_recipes: Vec<PotionTypeRecipe>,
    pub potion_container_recipes: Vec<PotionContainerChangeRecipe>,
    pub material_reducer_recipes: Vec<MaterialReducerRecipe>,
    pub clean_recipes: bool,
}

pub fn new(
    recipes: Vec<Recipe>,
    potion_type_recipes: Vec<PotionTypeRecipe>,
    potion_container_recipes: Vec<PotionContainerChangeRecipe>,
    material_reducer_recipes: Vec<MaterialReducerRecipe>,
    clean_recipes: bool,
) -> CraftingData {
    CraftingData {
        recipes,
        potion_type_recipes,
        potion_container_recipes,
        material_reducer_recipes,
        clean_recipes,
    }
}

impl Packet for CraftingData {
    fn id(&self) -> u16 {
        BedrockPacketType::IDCraftingData.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_var_u32(self.recipes.len() as u32);
        for recipe in self.recipes.iter_mut() {
            stream.put_var_i32(recipe.get_selected_type_id());
            recipe.write(&mut stream);
        }
        stream.put_var_u32(self.potion_type_recipes.len() as u32);
        for potion_type_recipe in &self.potion_type_recipes {
            stream.put_var_i32(potion_type_recipe.get_input_item_id());
            stream.put_var_i32(potion_type_recipe.get_input_item_meta());
            stream.put_var_i32(potion_type_recipe.get_ingredient_item_id());
            stream.put_var_i32(potion_type_recipe.get_ingredient_item_meta());
            stream.put_var_i32(potion_type_recipe.get_output_item_id());
            stream.put_var_i32(potion_type_recipe.get_output_item_meta());
        }
        stream.put_var_u32(self.potion_container_recipes.len() as u32);
        for potion_container_recipe in &self.potion_container_recipes {
            stream.put_var_i32(potion_container_recipe.input_item_id());
            stream.put_var_i32(potion_container_recipe.ingredient_item_id());
            stream.put_var_i32(potion_container_recipe.output_item_id());
        }
        stream.put_var_u32(self.material_reducer_recipes.len() as u32);
        for recipe in &self.material_reducer_recipes {
            stream.put_var_i32((recipe.get_input_item_id() << 16) | recipe.get_input_item_meta());
            stream.put_var_u32(recipe.get_outputs().len() as u32);
            for output in recipe.get_outputs() {
                stream.put_var_i32(output.get_item_id());
                stream.put_var_i32(output.get_count());
            }
        }
        stream.put_bool(self.clean_recipes);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> CraftingData {
        let recipe_count = stream.get_var_u32();
        let mut previous_type = 100; // 100 = none (I made it up)
        let mut recipes = Vec::new();
        for _ in 0..recipe_count {
            let recipe_type = stream.get_var_i32();
            recipes.push(match recipe_type {
                Self::ENTRY_SHAPELESS
                | Self::ENTRY_USER_DATA_SHAPELESS
                | Self::ENTRY_SHAPELESS_CHEMISTRY => {
                    Recipe::Shapeless(ShapelessRecipe::read(recipe_type, stream))
                }
                Self::ENTRY_SHAPED | Self::ENTRY_SHAPED_CHEMISTRY => {
                    Recipe::Shaped(ShapedRecipe::read(recipe_type, stream))
                }
                Self::ENTRY_FURNACE | Self::ENTRY_FURNACE_DATA => {
                    Recipe::Furnace(FurnaceRecipe::read(recipe_type, stream))
                }
                Self::ENTRY_MULTI => Recipe::Multi(MultiRecipe::read(recipe_type, stream)),
                Self::ENTRY_SMITHING_TRANSFORM => {
                    Recipe::SmitingTransform(SmithingTransformRecipe::read(recipe_type, stream))
                }
                Self::ENTRY_SMITHING_TRIM => {
                    Recipe::SmithingTrim(SmithingTrimRecipe::read(recipe_type, stream))
                }
                _ => {
                    panic!(
                        "Unhandled recipe type {} (previous was {})",
                        recipe_type, previous_type
                    );
                }
            });
            previous_type = recipe_type;
        }
        let mut count = stream.get_var_u32();
        let mut potion_type_recipes = Vec::new();
        for _ in 0..count {
            let input_item_id = stream.get_var_i32();
            let input_item_meta = stream.get_var_i32();
            let ingredient_item_id = stream.get_var_i32();
            let ingredient_item_meta = stream.get_var_i32();
            let output_item_id = stream.get_var_i32();
            let output_item_meta = stream.get_var_i32();

            potion_type_recipes.push(PotionTypeRecipe {
                input_item_id,
                input_item_meta,
                ingredient_item_id,
                ingredient_item_meta,
                output_item_id,
                output_item_meta,
            });
        }
        count = stream.get_var_u32();
        let mut potion_container_recipes = Vec::new();
        for _ in 0..count {
            let input_item_id = stream.get_var_i32();
            let ingredient_item_id = stream.get_var_i32();
            let output_item_id = stream.get_var_i32();

            potion_container_recipes.push(PotionContainerChangeRecipe {
                input_item_id,
                ingredient_item_id,
                output_item_id,
            });
        }
        count = stream.get_var_u32();
        let mut material_reducer_recipes = Vec::new();
        for _ in 0..count {
            let input_id_and_data = stream.get_var_i32();
            let input_item_id = input_id_and_data >> 16;
            let input_item_meta = input_id_and_data & 0x7fff;
            let mut outputs = Vec::new();
            let output_count = stream.get_var_u32();
            for _ in 0..output_count {
                let item_id = stream.get_var_i32();
                let count = stream.get_var_i32();
                outputs.push(MaterialReducerRecipeOutput { item_id, count });
            }
            material_reducer_recipes.push(MaterialReducerRecipe {
                input_item_id,
                input_item_meta,
                outputs,
            });
        }
        let clean_recipes = stream.get_bool();

        CraftingData {
            recipes,
            potion_type_recipes,
            potion_container_recipes,
            material_reducer_recipes,
            clean_recipes,
        }
    }

    fn debug(&self) {
        println!("Recipes: {:?}", self.recipes);
        println!("Potion Type Recipes: {:?}", self.potion_type_recipes);
        println!(
            "Potion Container Change Recipe: {:?}",
            self.potion_container_recipes
        );
        println!(
            "Material Reducer Recipes: {:?}",
            self.material_reducer_recipes
        );
        println!("Clean Recipes: {}", self.clean_recipes);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl CraftingData {
    pub const ENTRY_SHAPELESS: i32 = 0;
    pub const ENTRY_SHAPED: i32 = 1;
    pub const ENTRY_FURNACE: i32 = 2;
    pub const ENTRY_FURNACE_DATA: i32 = 3;
    pub const ENTRY_MULTI: i32 = 4;
    pub const ENTRY_USER_DATA_SHAPELESS: i32 = 5;
    pub const ENTRY_SHAPELESS_CHEMISTRY: i32 = 6;
    pub const ENTRY_SHAPED_CHEMISTRY: i32 = 7;
    pub const ENTRY_SMITHING_TRANSFORM: i32 = 8;
    pub const ENTRY_SMITHING_TRIM: i32 = 9;
}
