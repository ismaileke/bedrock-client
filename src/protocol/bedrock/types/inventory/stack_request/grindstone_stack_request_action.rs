use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::inventory::stack_request::item_stack_request_action::ItemStackRequestAction;
use crate::protocol::bedrock::types::inventory::stack_request::item_stack_request_action_type::ItemStackRequestActionType;

#[derive(Debug)]
pub struct GrindstoneStackRequestAction {
    recipe_id: u32,
    repair_cost: i32, //WHY
    repetitions: u8
}

impl GrindstoneStackRequestAction {
    pub fn new(recipe_id: u32, repair_cost: i32, repetitions: u8) -> GrindstoneStackRequestAction {
        GrindstoneStackRequestAction{ recipe_id, repair_cost, repetitions }
    }

    pub fn read(stream: &mut Stream) -> GrindstoneStackRequestAction {
        let recipe_id = PacketSerializer::read_recipe_net_id(stream);
        let repair_cost = stream.get_var_int();
        let repetitions = stream.get_byte();

        GrindstoneStackRequestAction{ recipe_id, repair_cost, repetitions }
    }
}

impl ItemStackRequestAction for GrindstoneStackRequestAction {
    fn get_type_id(&self) -> u8 {
        ItemStackRequestActionType::CRAFTING_GRINDSTONE
    }

    fn write(&mut self, stream: &mut Stream) {
        PacketSerializer::write_recipe_net_id(stream, self.recipe_id);
        stream.put_var_int(self.repair_cost);
        stream.put_byte(self.repetitions);
    }
}


