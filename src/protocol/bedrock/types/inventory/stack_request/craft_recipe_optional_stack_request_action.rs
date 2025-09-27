use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::inventory::stack_request::item_stack_request_action::ItemStackRequestAction;
use crate::protocol::bedrock::types::inventory::stack_request::item_stack_request_action_type::ItemStackRequestActionType;

#[derive(Debug)]
pub struct CraftRecipeOptionalStackRequestAction {
    recipe_id: u32,
    filter_string_index: u32
}

impl CraftRecipeOptionalStackRequestAction {
    pub fn new(recipe_id: u32, filter_string_index: u32) -> CraftRecipeOptionalStackRequestAction {
        CraftRecipeOptionalStackRequestAction{ recipe_id, filter_string_index }
    }

    pub fn read(stream: &mut Stream) -> CraftRecipeOptionalStackRequestAction {
        let recipe_id = PacketSerializer::read_recipe_net_id(stream);
        let filter_string_index = stream.get_l_int();

        CraftRecipeOptionalStackRequestAction{ recipe_id, filter_string_index }
    }
}

impl ItemStackRequestAction for CraftRecipeOptionalStackRequestAction {
    fn get_type_id(&self) -> u8 {
        ItemStackRequestActionType::CRAFTING_RECIPE_OPTIONAL
    }

    fn write(&mut self, stream: &mut Stream) {
        PacketSerializer::write_recipe_net_id(stream, self.recipe_id);
        stream.put_l_int(self.filter_string_index);
    }
}


