use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

#[derive(serde::Serialize, Debug)]
pub struct GatheringJoinInfo {
    pub experience_id: String,
    pub experience_name: String,
    pub experience_world_id: String,
    pub experience_world_name: String,
    pub creator_id: String,
    pub store_id: String
}

impl GatheringJoinInfo {
    pub fn read(stream: &mut Stream) -> GatheringJoinInfo {
        let experience_id = PacketSerializer::get_string(stream);
        let experience_name = PacketSerializer::get_string(stream);
        let experience_world_id = PacketSerializer::get_string(stream);
        let experience_world_name = PacketSerializer::get_string(stream);
        let creator_id = PacketSerializer::get_string(stream);
        let store_id = PacketSerializer::get_string(stream);

        GatheringJoinInfo { experience_id, experience_name, experience_world_id, experience_world_name, creator_id, store_id }
    }

    pub fn write(&self, stream: &mut Stream) {
        PacketSerializer::put_string(stream, self.experience_id.clone());
        PacketSerializer::put_string(stream, self.experience_name.clone());
        PacketSerializer::put_string(stream, self.experience_world_id.clone());
        PacketSerializer::put_string(stream, self.experience_world_name.clone());
        PacketSerializer::put_string(stream, self.creator_id.clone());
        PacketSerializer::put_string(stream, self.store_id.clone());
    }
}