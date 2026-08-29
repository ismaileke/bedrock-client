use binary_utils::binary::{Reader, Writer};
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

#[derive(serde::Serialize, Debug)]
pub struct GatheringJoinInfo {
    pub experience_id: String,
    pub experience_name: String,
    pub experience_world_id: Option<String>,
    pub experience_world_name: Option<String>,
    pub creator_id: String,
    pub target_id: Option<String>,
    pub scenario_id: Option<String>,
    pub server_id: Option<String>,
}

impl GatheringJoinInfo {
    pub fn read(stream: &mut Reader) -> GatheringJoinInfo {
        let experience_id = PacketSerializer::get_uuid(stream);
        let experience_name = PacketSerializer::get_string(stream);
        let experience_world_id = PacketSerializer::read_optional(stream, |s| PacketSerializer::get_uuid(s));
        let experience_world_name = PacketSerializer::read_optional(stream, |s| PacketSerializer::get_string(s));
        let creator_id = PacketSerializer::get_string(stream);
        let target_id = PacketSerializer::read_optional(stream, |s| PacketSerializer::get_uuid(s));
        let scenario_id = PacketSerializer::read_optional(stream, |s| PacketSerializer::get_string(s));
        let server_id = PacketSerializer::read_optional(stream, |s| PacketSerializer::get_string(s));

        GatheringJoinInfo { experience_id, experience_name, experience_world_id, experience_world_name, creator_id, target_id, scenario_id, server_id }
    }

    pub fn write(&self, stream: &mut Writer) {
        PacketSerializer::put_uuid(stream, &self.experience_id);
        PacketSerializer::put_string(stream, &self.experience_name);
        PacketSerializer::write_optional(stream, &self.experience_world_id, |s, v| PacketSerializer::put_uuid(s, v));
        PacketSerializer::write_optional(stream, &self.experience_world_name, |s, v| PacketSerializer::put_string(s, v));
        PacketSerializer::put_string(stream, &self.creator_id);
        PacketSerializer::write_optional(stream, &self.target_id, |s, v| PacketSerializer::put_uuid(s, v));
        PacketSerializer::write_optional(stream, &self.scenario_id, |s, v| PacketSerializer::put_string(s, v));
        PacketSerializer::write_optional(stream, &self.server_id, |s, v| PacketSerializer::put_string(s, v));
    }
}