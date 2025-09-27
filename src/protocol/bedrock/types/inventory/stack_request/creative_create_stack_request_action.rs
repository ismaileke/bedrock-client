use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::inventory::stack_request::item_stack_request_action::ItemStackRequestAction;
use crate::protocol::bedrock::types::inventory::stack_request::item_stack_request_action_type::ItemStackRequestActionType;

#[derive(Debug)]
pub struct CreativeCreateStackRequestAction {
    creative_item_id: u32,
    repetitions: u8
}

impl CreativeCreateStackRequestAction {
    pub fn new(creative_item_id: u32, repetitions: u8) -> CreativeCreateStackRequestAction {
        CreativeCreateStackRequestAction{ creative_item_id, repetitions }
    }

    pub fn read(stream: &mut Stream) -> CreativeCreateStackRequestAction {
        let creative_item_id = PacketSerializer::read_creative_item_net_id(stream);
        let repetitions = stream.get_byte();

        CreativeCreateStackRequestAction{ creative_item_id, repetitions }
    }
}

impl ItemStackRequestAction for CreativeCreateStackRequestAction {
    fn get_type_id(&self) -> u8 {
        ItemStackRequestActionType::CREATIVE_CREATE
    }

    fn write(&mut self, stream: &mut Stream) {
        PacketSerializer::write_creative_item_net_id(stream, self.creative_item_id);
        stream.put_byte(self.repetitions);
    }
}


