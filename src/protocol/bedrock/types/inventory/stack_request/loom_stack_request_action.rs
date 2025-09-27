use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::inventory::stack_request::item_stack_request_action::ItemStackRequestAction;
use crate::protocol::bedrock::types::inventory::stack_request::item_stack_request_action_type::ItemStackRequestActionType;

#[derive(Debug)]
pub struct LoomStackRequestAction {
    pattern_id: String,
    repetitions: u8
}

impl LoomStackRequestAction {
    pub fn new(pattern_id: String, repetitions: u8) -> LoomStackRequestAction {
        LoomStackRequestAction{ pattern_id, repetitions }
    }

    pub fn read(stream: &mut Stream) -> LoomStackRequestAction {
        let pattern_id = PacketSerializer::get_string(stream);
        let repetitions = stream.get_byte();

        LoomStackRequestAction{ pattern_id, repetitions }
    }
}

impl ItemStackRequestAction for LoomStackRequestAction {
    fn get_type_id(&self) -> u8 {
        ItemStackRequestActionType::CRAFTING_LOOM
    }

    fn write(&mut self, stream: &mut Stream) {
        PacketSerializer::put_string(stream, self.pattern_id.clone());
        stream.put_byte(self.repetitions);
    }
}


