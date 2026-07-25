use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct PresenceConfig {
    experience_name: String,
    world_name: String,
}

impl PresenceConfig {
    pub fn new(experience_name: String, world_name: String) -> PresenceConfig {
        PresenceConfig { experience_name, world_name }
    }

    pub fn read(stream: &mut Stream) -> PresenceConfig {
        let experience_name = PacketSerializer::get_string(stream);
        let world_name = PacketSerializer::get_string(stream);
        PresenceConfig { experience_name, world_name }
    }

    pub fn write(&self, stream: &mut Stream) {
        PacketSerializer::put_string(stream, self.experience_name.clone());
        PacketSerializer::put_string(stream, self.world_name.clone());
    }
}
