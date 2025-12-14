use crate::protocol::bedrock::crafting_data::CraftingData;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::inventory::item_stack::ItemStack;
use crate::protocol::bedrock::types::recipe::recipe_ingredient::RecipeIngredient;
use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct SmithingTransformRecipe {
    type_id: i32,
    recipe_id: String,
    template: RecipeIngredient,
    input: RecipeIngredient,
    addition: RecipeIngredient,
    output: ItemStack,
    block_name: String,
    recipe_net_id: u32,
}

impl SmithingTransformRecipe {
    pub fn new(
        type_id: i32,
        recipe_id: String,
        template: RecipeIngredient,
        input: RecipeIngredient,
        addition: RecipeIngredient,
        output: ItemStack,
        block_name: String,
        recipe_net_id: u32,
    ) -> SmithingTransformRecipe {
        SmithingTransformRecipe {
            type_id,
            recipe_id,
            template,
            input,
            addition,
            output,
            block_name,
            recipe_net_id,
        }
    }

    pub fn get_type_ids() -> Vec<i32> {
        Vec::from([CraftingData::ENTRY_SMITHING_TRANSFORM])
    }

    pub fn get_selected_type_id(&self) -> i32 {
        self.type_id
    }

    pub fn read(type_id: i32, stream: &mut Stream) -> SmithingTransformRecipe {
        let recipe_id = PacketSerializer::get_string(stream);
        let template = PacketSerializer::get_recipe_ingredient(stream);
        let input = PacketSerializer::get_recipe_ingredient(stream);
        let addition = PacketSerializer::get_recipe_ingredient(stream);
        let output = PacketSerializer::get_item_stack_without_stack_id(stream);
        let block_name = PacketSerializer::get_string(stream);
        let recipe_net_id = PacketSerializer::read_recipe_net_id(stream);

        SmithingTransformRecipe {
            type_id,
            recipe_id,
            template,
            input,
            addition,
            output,
            block_name,
            recipe_net_id,
        }
    }

    pub fn write(&mut self, stream: &mut Stream) {
        PacketSerializer::put_string(stream, self.recipe_id.clone());
        PacketSerializer::put_recipe_ingredient(stream, &mut self.template);
        PacketSerializer::put_recipe_ingredient(stream, &mut self.input);
        PacketSerializer::put_recipe_ingredient(stream, &mut self.addition);
        PacketSerializer::put_item_stack_without_stack_id(stream, &self.output);
        PacketSerializer::put_string(stream, self.block_name.clone());
        PacketSerializer::write_recipe_net_id(stream, self.recipe_net_id);
    }
}
