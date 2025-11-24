use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

#[derive(serde::Serialize, Debug)]
pub struct CraftRecipeOptionalStackRequestAction {
    recipe_id: u32,
    filter_string_index: i32
}

impl CraftRecipeOptionalStackRequestAction {
    pub fn new(recipe_id: u32, filter_string_index: i32) -> CraftRecipeOptionalStackRequestAction {
        CraftRecipeOptionalStackRequestAction{ recipe_id, filter_string_index }
    }

    pub fn read(stream: &mut Stream) -> CraftRecipeOptionalStackRequestAction {
        let recipe_id = PacketSerializer::read_recipe_net_id(stream);
        let filter_string_index = stream.get_i32_le();

        CraftRecipeOptionalStackRequestAction{ recipe_id, filter_string_index }
    }

    pub fn write(&mut self, stream: &mut Stream) {
        PacketSerializer::write_recipe_net_id(stream, self.recipe_id);
        stream.put_i32_le(self.filter_string_index);
    }
}
