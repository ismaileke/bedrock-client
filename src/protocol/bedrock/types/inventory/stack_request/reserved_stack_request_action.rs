use binary_utils::binary::{Reader, Writer};
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

#[derive(serde::Serialize, Debug)]
pub struct ReservedStackRequestAction {
    pub reserved_id: String,
    pub num_crafts: u8
}

impl ReservedStackRequestAction {
    pub fn new(reserved_id: String, num_crafts: u8) -> ReservedStackRequestAction {
        ReservedStackRequestAction { reserved_id, num_crafts }
    }

    pub fn read(stream: &mut Reader) -> ReservedStackRequestAction {
        let reserved_id = PacketSerializer::get_string(stream);
        let num_crafts = stream.get_u8();

        ReservedStackRequestAction { reserved_id, num_crafts }
    }

    pub fn write(&self, stream: &mut Writer) {
        PacketSerializer::put_string(stream, &self.reserved_id);
        stream.put_u8(self.num_crafts);
    }
}
