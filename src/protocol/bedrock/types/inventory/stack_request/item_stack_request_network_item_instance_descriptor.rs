use binary_utils::binary::{Reader, Writer};
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::recipe::recipe_ingredient::RecipeIngredient;

#[derive(serde::Serialize, Debug)]
pub struct ItemStackRequestNetworkItemInstanceDescriptor {
    pub ingredient: RecipeIngredient,
    pub block_runtime_id: u32,
    pub raw_extra_data: Vec<u8>
}

impl ItemStackRequestNetworkItemInstanceDescriptor {
    pub fn new(ingredient: RecipeIngredient, block_runtime_id: u32, raw_extra_data: Vec<u8>) -> ItemStackRequestNetworkItemInstanceDescriptor {
        ItemStackRequestNetworkItemInstanceDescriptor { ingredient, block_runtime_id, raw_extra_data }
    }

    pub fn read(stream: &mut Reader) -> ItemStackRequestNetworkItemInstanceDescriptor {
        let ingredient = PacketSerializer::get_recipe_ingredient(stream);
        let block_runtime_id = stream.get_var_u32();
        let raw_extra_data = PacketSerializer::get_byte_string(stream);
        ItemStackRequestNetworkItemInstanceDescriptor { ingredient, block_runtime_id, raw_extra_data }
    }

    pub fn write(&self, stream: &mut Writer) {
        PacketSerializer::put_recipe_ingredient(stream, &self.ingredient);
        stream.put_var_u32(self.block_runtime_id);
        PacketSerializer::put_byte_string(stream, &self.raw_extra_data);
    }
}
