use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct TrimPattern {
    pub item_id: String,
    pub pattern_id: String,
}

impl TrimPattern {
    pub fn new(item_id: String, pattern_id: String) -> TrimPattern {
        TrimPattern { item_id, pattern_id }
    }

    pub fn read(stream: &mut Reader) -> TrimPattern {
        let item_id = PacketSerializer::get_string(stream);
        let pattern_id = PacketSerializer::get_string(stream);

        TrimPattern { item_id, pattern_id }
    }

    pub fn write(&self, stream: &mut Writer) {
        PacketSerializer::put_string(stream, &self.item_id);
        PacketSerializer::put_string(stream, &self.pattern_id);
    }
}
