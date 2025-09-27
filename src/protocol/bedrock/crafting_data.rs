use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::types::recipe::furnace_recipe::FurnaceRecipe;
use crate::protocol::bedrock::types::recipe::material_reducer_recipe::MaterialReducerRecipe;
use crate::protocol::bedrock::types::recipe::material_reducer_recipe_output::MaterialReducerRecipeOutput;
use crate::protocol::bedrock::types::recipe::multi_recipe::MultiRecipe;
use crate::protocol::bedrock::types::recipe::potion_container_change_recipe::PotionContainerChangeRecipe;
use crate::protocol::bedrock::types::recipe::potion_type_recipe::PotionTypeRecipe;
use crate::protocol::bedrock::types::recipe::recipe_with_type_id::RecipeWithTypeId;
use crate::protocol::bedrock::types::recipe::shaped_recipe::ShapedRecipe;
use crate::protocol::bedrock::types::recipe::shapeless_recipe::ShapelessRecipe;
use crate::protocol::bedrock::types::recipe::smithing_transform_recipe::SmithingTransformRecipe;
use crate::protocol::bedrock::types::recipe::smithing_trim_recipe::SmithingTrimRecipe;

pub struct CraftingData {
    pub recipes_with_type_ids: Vec<Box<dyn RecipeWithTypeId>>,
    pub potion_type_recipes: Vec<PotionTypeRecipe>,
    pub potion_container_recipes: Vec<PotionContainerChangeRecipe>,
    pub material_reducer_recipes: Vec<MaterialReducerRecipe>,
    pub clean_recipes: bool
}

pub fn new(
    recipes_with_type_ids: Vec<Box<dyn RecipeWithTypeId>>,
    potion_type_recipes: Vec<PotionTypeRecipe>,
    potion_container_recipes: Vec<PotionContainerChangeRecipe>,
    material_reducer_recipes: Vec<MaterialReducerRecipe>,
    clean_recipes: bool
) -> CraftingData {
    CraftingData { recipes_with_type_ids, potion_type_recipes, potion_container_recipes, material_reducer_recipes, clean_recipes }
}

impl CraftingData {
    pub  const ENTRY_SHAPELESS: i32 = 0;
    pub  const ENTRY_SHAPED: i32 = 1;
    pub  const ENTRY_FURNACE: i32 = 2;
    pub  const ENTRY_FURNACE_DATA: i32 = 3;
    pub  const ENTRY_MULTI: i32 = 4;
    pub  const ENTRY_USER_DATA_SHAPELESS: i32 = 5;
    pub  const ENTRY_SHAPELESS_CHEMISTRY: i32 = 6;
    pub  const ENTRY_SHAPED_CHEMISTRY: i32 = 7;
    pub  const ENTRY_SMITHING_TRANSFORM: i32 = 8;
    pub  const ENTRY_SMITHING_TRIM: i32 = 9;
}

impl Packet for CraftingData {
    fn id(&self) -> u16 {
        BedrockPacketType::IDCraftingData.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        stream.put_unsigned_var_int(self.recipes_with_type_ids.len() as u32);
        for recipes_with_type_id in self.recipes_with_type_ids.iter_mut() {
            stream.put_var_int(recipes_with_type_id.get_selected_type_id());
            recipes_with_type_id.write(&mut stream);
        }
        stream.put_var_int(self.potion_type_recipes.len() as i32);
        for potion_type_recipe in &self.potion_type_recipes {
            stream.put_var_int(potion_type_recipe.get_input_item_id());
            stream.put_var_int(potion_type_recipe.get_input_item_meta());
            stream.put_var_int(potion_type_recipe.get_ingredient_item_id());
            stream.put_var_int(potion_type_recipe.get_ingredient_item_meta());
            stream.put_var_int(potion_type_recipe.get_output_item_id());
            stream.put_var_int(potion_type_recipe.get_output_item_meta());
        }
        stream.put_unsigned_var_int(self.potion_container_recipes.len() as u32);
        for potion_container_recipe in &self.potion_container_recipes {
            stream.put_var_int(potion_container_recipe.input_item_id());
            stream.put_var_int(potion_container_recipe.ingredient_item_id());
            stream.put_var_int(potion_container_recipe.output_item_id());
        }
        stream.put_unsigned_var_int(self.material_reducer_recipes.len() as u32);
        for recipe in &self.material_reducer_recipes {
            stream.put_var_int((recipe.get_input_item_id() << 16) | recipe.get_input_item_meta());
            stream.put_unsigned_var_int(recipe.get_outputs().len() as u32);
            for output in recipe.get_outputs() {
                stream.put_var_int(output.get_item_id());
                stream.put_var_int(output.get_count());
            }
        }
        stream.put_bool(self.clean_recipes);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> CraftingData {
        let mut stream = Stream::new(bytes, 0);

        let recipe_count = stream.get_unsigned_var_int();
        let mut previous_type = 100; // 100 = none (I made it up)
        let mut recipes_with_type_ids = Vec::new();
        for _ in 0..recipe_count {
            let recipe_type = stream.get_var_int();
            recipes_with_type_ids.push(
                match recipe_type {
                    Self::ENTRY_SHAPELESS | Self::ENTRY_USER_DATA_SHAPELESS | Self::ENTRY_SHAPELESS_CHEMISTRY => {
                        Box::new(ShapelessRecipe::read(recipe_type, &mut stream)) as Box<dyn RecipeWithTypeId>
                    },
                    Self::ENTRY_SHAPED | Self::ENTRY_SHAPED_CHEMISTRY => {
                        Box::new(ShapedRecipe::read(recipe_type, &mut stream)) as Box<dyn RecipeWithTypeId>
                    },
                    Self::ENTRY_FURNACE | Self::ENTRY_FURNACE_DATA => {
                        Box::new(FurnaceRecipe::read(recipe_type, &mut stream)) as Box<dyn RecipeWithTypeId>
                    },
                    Self::ENTRY_MULTI => {
                        Box::new(MultiRecipe::read(recipe_type, &mut stream)) as Box<dyn RecipeWithTypeId>
                    },
                    Self::ENTRY_SMITHING_TRANSFORM => {
                        Box::new(SmithingTransformRecipe::read(recipe_type, &mut stream)) as Box<dyn RecipeWithTypeId>
                    },
                    Self::ENTRY_SMITHING_TRIM => {
                        Box::new(SmithingTrimRecipe::read(recipe_type, &mut stream)) as Box<dyn RecipeWithTypeId>
                    },
                    _ => {
                        panic!("Unhandled recipe type {} (previous was {})", recipe_type, previous_type);
                    }
                }
            );
            previous_type = recipe_type;
        }
        let mut count = stream.get_unsigned_var_int();
        let mut potion_type_recipes = Vec::new();
        for _ in 0..count {
            let input_item_id = stream.get_var_int();
            let input_item_meta = stream.get_var_int();
            let ingredient_item_id = stream.get_var_int();
            let ingredient_item_meta = stream.get_var_int();
            let output_item_id = stream.get_var_int();
            let output_item_meta = stream.get_var_int();

            potion_type_recipes.push(PotionTypeRecipe{
                input_item_id,
                input_item_meta,
                ingredient_item_id,
                ingredient_item_meta,
                output_item_id,
                output_item_meta
            });
        }
        count = stream.get_unsigned_var_int();
        let mut potion_container_recipes = Vec::new();
        for _ in 0..count {
            let input_item_id = stream.get_var_int();
            let ingredient_item_id = stream.get_var_int();
            let output_item_id = stream.get_var_int();

            potion_container_recipes.push(PotionContainerChangeRecipe{ input_item_id, ingredient_item_id, output_item_id });
        }
        count = stream.get_unsigned_var_int();
        let mut material_reducer_recipes = Vec::new();
        for _ in 0..count {
            let input_id_and_data = stream.get_var_int();
            let input_item_id = input_id_and_data >> 16;
            let input_item_meta = input_id_and_data & 0x7fff;
            let mut outputs = Vec::new();
            let output_count = stream.get_unsigned_var_int();
            for _ in 0..output_count {
                let item_id = stream.get_var_int();
                let count = stream.get_var_int();
                outputs.push(MaterialReducerRecipeOutput{ item_id, count });
            }
            material_reducer_recipes.push(MaterialReducerRecipe{ input_item_id, input_item_meta, outputs });
        }
        let clean_recipes = stream.get_bool();

        CraftingData { recipes_with_type_ids, potion_type_recipes, potion_container_recipes, material_reducer_recipes, clean_recipes }
    }

    fn debug(&self) {
        println!("Recipes With Type IDs: {:?}", self.recipes_with_type_ids);
        println!("Potion Type Recipes: {:?}", self.potion_type_recipes);
        println!("Potion Container Change Recipe: {:?}", self.potion_container_recipes);
        println!("Material Reducer Recipes: {:?}", self.material_reducer_recipes);
        println!("Clean Recipes: {}", self.clean_recipes);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
