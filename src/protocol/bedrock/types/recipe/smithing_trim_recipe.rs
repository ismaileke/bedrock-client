use binary_utils::binary::Stream;
use crate::protocol::bedrock::crafting_data::CraftingData;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::recipe::recipe_ingredient::RecipeIngredient;
use crate::protocol::bedrock::types::recipe::recipe_with_type_id::RecipeWithTypeId;

#[derive(Debug)]
pub struct SmithingTrimRecipe {
    type_id: i32,
    recipe_id: String,
    template: RecipeIngredient,
    input: RecipeIngredient,
    addition: RecipeIngredient,
    block_name: String,
    recipe_net_id: u32
}

impl SmithingTrimRecipe {
    pub fn new(type_id: i32, recipe_id: String, template: RecipeIngredient, input: RecipeIngredient, addition: RecipeIngredient, block_name: String, recipe_net_id: u32) -> SmithingTrimRecipe {
        SmithingTrimRecipe{ type_id, recipe_id, template, input, addition, block_name, recipe_net_id }
    }

    pub fn get_type_ids() -> Vec<i32> {
        Vec::from([
            CraftingData::ENTRY_SMITHING_TRIM
        ])
    }

    pub fn read(type_id: i32, stream: &mut Stream) -> SmithingTrimRecipe {
        let recipe_id = PacketSerializer::get_string(stream);
        let template = PacketSerializer::get_recipe_ingredient(stream);
        let input = PacketSerializer::get_recipe_ingredient(stream);
        let addition = PacketSerializer::get_recipe_ingredient(stream);
        let block_name = PacketSerializer::get_string(stream);
        let recipe_net_id = PacketSerializer::read_recipe_net_id(stream);

        SmithingTrimRecipe{ type_id, recipe_id, template, input, addition, block_name, recipe_net_id }
    }
}

impl RecipeWithTypeId for SmithingTrimRecipe {
    fn get_selected_type_id(&self) -> i32 {
        self.type_id
    }

    fn write(&mut self, stream: &mut Stream) {
        PacketSerializer::put_string(stream, self.recipe_id.clone());
        PacketSerializer::put_recipe_ingredient(stream, &mut self.template);
        PacketSerializer::put_recipe_ingredient(stream, &mut self.input);
        PacketSerializer::put_recipe_ingredient(stream, &mut self.addition);
        PacketSerializer::put_string(stream, self.block_name.clone());
        PacketSerializer::write_recipe_net_id(stream, self.recipe_net_id);
    }
}