use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::inventory::stack_request::item_stack_request_action::ItemStackRequestAction;
use crate::protocol::bedrock::types::inventory::stack_request::item_stack_request_action_type::ItemStackRequestActionType;

#[derive(Debug)]
pub struct MineBlockStackRequestAction {
    hotbar_slot: i32,
    predicted_durability: i32,
    stack_id: i32
}

impl MineBlockStackRequestAction {
    pub fn new(hotbar_slot: i32, predicted_durability: i32, stack_id: i32) -> MineBlockStackRequestAction {
        MineBlockStackRequestAction{ hotbar_slot, predicted_durability, stack_id }
    }

    pub fn read(stream: &mut Stream) -> MineBlockStackRequestAction {
        let hotbar_slot = stream.get_var_int();
        let predicted_durability = stream.get_var_int();
        let stack_id = PacketSerializer::read_item_stack_net_id_variant(stream);

        MineBlockStackRequestAction{ hotbar_slot, predicted_durability, stack_id }
    }
}

impl ItemStackRequestAction for MineBlockStackRequestAction {
    fn get_type_id(&self) -> u8 {
        ItemStackRequestActionType::MINE_BLOCK
    }

    fn write(&mut self, stream: &mut Stream) {
        stream.put_var_int(self.hotbar_slot);
        stream.put_var_int(self.predicted_durability);
        PacketSerializer::write_item_stack_net_id_variant(stream, self.stack_id);
    }
}


