use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

#[derive(serde::Serialize, Debug)]
pub struct TrimPattern {
    item_id: String,
    pattern_id: String
}

impl TrimPattern {
    pub fn new(item_id: String, pattern_id: String) -> TrimPattern {
        TrimPattern{ item_id, pattern_id }
    }

    pub fn read(stream: &mut Stream) -> TrimPattern {
        let item_id = PacketSerializer::get_string(stream);
        let pattern_id = PacketSerializer::get_string(stream);

        TrimPattern{ item_id, pattern_id }
    }

    pub fn write(&self, stream: &mut Stream) {
        PacketSerializer::put_string(stream, self.item_id.clone());
        PacketSerializer::put_string(stream, self.pattern_id.clone());
    }
}