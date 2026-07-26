use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct PresenceInfo {
    experience_name: Option<String>,
    world_name: Option<String>,
    rich_presence_id: String,
}

impl PresenceInfo {
    pub fn new(experience_name: Option<String>, world_name: Option<String>, rich_presence_id: String,) -> PresenceInfo {
        PresenceInfo { experience_name, world_name, rich_presence_id }
    }

    pub fn read(stream: &mut Stream) -> PresenceInfo {
        let experience_name = PacketSerializer::read_optional(stream, |stream| PacketSerializer::get_string(stream));
        let world_name = PacketSerializer::read_optional(stream, |stream| PacketSerializer::get_string(stream));
        let rich_presence_id = PacketSerializer::get_string(stream);
        PresenceInfo { experience_name, world_name, rich_presence_id }
    }

    pub fn write(&self, stream: &mut Stream) {
        PacketSerializer::write_optional(stream, &self.experience_name, |stream, value| PacketSerializer::put_string(stream, value.clone()));
        PacketSerializer::write_optional(stream, &self.world_name, |stream, value| PacketSerializer::put_string(stream, value.clone()));
        PacketSerializer::put_string(stream, self.rich_presence_id.clone());
    }
}
