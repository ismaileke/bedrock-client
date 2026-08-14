use binary_utils::binary::{Reader, Writer};
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

#[derive(serde::Serialize, Debug)]
pub struct GatheringJoinInfo {
    pub experience_id: String,
    pub experience_name: String,
    pub experience_world_id: String,
    pub experience_world_name: String,
    pub creator_id: String,
    pub target_id: String,
    pub scenario_id: String,
    pub server_id: String,
}

impl GatheringJoinInfo {
    pub fn read(stream: &mut Reader) -> GatheringJoinInfo {
        let experience_id = PacketSerializer::get_uuid(stream);
        let experience_name = PacketSerializer::get_string(stream);
        let experience_world_id = PacketSerializer::get_uuid(stream);
        let experience_world_name = PacketSerializer::get_string(stream);
        let creator_id = PacketSerializer::get_string(stream);
        let target_id = PacketSerializer::get_uuid(stream);
        let scenario_id = PacketSerializer::get_string(stream);
        let server_id = PacketSerializer::get_string(stream);

        GatheringJoinInfo { experience_id, experience_name, experience_world_id, experience_world_name, creator_id, target_id, scenario_id, server_id }
    }

    pub fn write(&self, stream: &mut Writer) {
        PacketSerializer::put_uuid(stream, &self.experience_id);
        PacketSerializer::put_string(stream, &self.experience_name);
        PacketSerializer::put_uuid(stream, &self.experience_world_id);
        PacketSerializer::put_string(stream, &self.experience_world_name);
        PacketSerializer::put_string(stream, &self.creator_id);
        PacketSerializer::put_uuid(stream, &self.target_id);
        PacketSerializer::put_string(stream, &self.scenario_id);
        PacketSerializer::put_string(stream, &self.server_id);
    }
}