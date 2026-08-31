use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct PresenceInfo {
    pub rich_presence_id: Option<String>
}

impl PresenceInfo {
    pub fn new(rich_presence_id: Option<String>) -> PresenceInfo {
        PresenceInfo { rich_presence_id }
    }

    pub fn read(stream: &mut Reader) -> PresenceInfo {
        let rich_presence_id = PacketSerializer::read_optional(stream, |stream| PacketSerializer::get_string(stream));
        PresenceInfo { rich_presence_id }
    }

    pub fn write(&self, stream: &mut Writer) {
        PacketSerializer::write_optional(stream, &self.rich_presence_id, |stream, value| PacketSerializer::put_string(stream, value));
    }
}
