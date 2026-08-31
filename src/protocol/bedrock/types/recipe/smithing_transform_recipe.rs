use crate::protocol::bedrock::crafting_data::CraftingData;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::inventory::item_stack::ItemStack;
use crate::protocol::bedrock::types::recipe::recipe_ingredient::RecipeIngredient;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct SmithingTransformRecipe {
    pub type_id: i32,
    pub recipe_id: String,
    pub template: RecipeIngredient,
    pub input: RecipeIngredient,
    pub addition: RecipeIngredient,
    pub output: ItemStack,
    pub block_name: String,
    pub recipe_net_id: i32,
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
        recipe_net_id: i32,
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

    pub fn read(type_id: i32, stream: &mut Reader) -> SmithingTransformRecipe {
        let recipe_id = PacketSerializer::get_string(stream);
        let template = RecipeIngredient::read(stream);
        let input = RecipeIngredient::read(stream);
        let addition = RecipeIngredient::read(stream);
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

    pub fn write(&mut self, stream: &mut Writer) {
        PacketSerializer::put_string(stream, &self.recipe_id);
        self.template.write(stream);
        self.input.write(stream);
        self.addition.write(stream);
        PacketSerializer::put_item_stack_without_stack_id(stream, &self.output);
        PacketSerializer::put_string(stream, &self.block_name);
        PacketSerializer::write_recipe_net_id(stream, self.recipe_net_id);
    }
}
