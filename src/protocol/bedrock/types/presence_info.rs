use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct PresenceInfo {
    experience_name: String,
    world_name: String,
}

impl PresenceInfo {
    pub fn new(experience_name: String, world_name: String) -> PresenceInfo {
        PresenceInfo { experience_name, world_name }
    }

    pub fn read(stream: &mut Stream) -> PresenceInfo {
        let experience_name = PacketSerializer::get_string(stream);
        let world_name = PacketSerializer::get_string(stream);
        PresenceInfo { experience_name, world_name }
    }

    pub fn write(&self, stream: &mut Stream) {
        PacketSerializer::put_string(stream, self.experience_name.clone());
        PacketSerializer::put_string(stream, self.world_name.clone());
    }
}
