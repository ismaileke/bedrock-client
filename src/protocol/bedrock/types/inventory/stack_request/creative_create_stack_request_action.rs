use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct CreativeCreateStackRequestAction {
    pub creative_item_id: u32,
    pub repetitions: u8,
}

impl CreativeCreateStackRequestAction {
    pub fn new(creative_item_id: u32, repetitions: u8) -> CreativeCreateStackRequestAction {
        CreativeCreateStackRequestAction { creative_item_id, repetitions }
    }

    pub fn read(stream: &mut Reader) -> CreativeCreateStackRequestAction {
        let creative_item_id = PacketSerializer::read_creative_item_net_id(stream);
        let repetitions = stream.get_u8();

        CreativeCreateStackRequestAction { creative_item_id, repetitions }
    }

    pub fn write(&self, stream: &mut Writer) {
        PacketSerializer::write_creative_item_net_id(stream, self.creative_item_id);
        stream.put_u8(self.repetitions);
    }
}
