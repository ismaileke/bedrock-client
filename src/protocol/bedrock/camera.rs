use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct Camera {
    pub camera_actor_unique_id: i64,
    pub player_actor_unique_id: i64,
}

impl Packet for Camera {
    fn id(&self) -> u16 {
        BedrockPacketType::IDCamera.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_actor_unique_id(stream, self.camera_actor_unique_id);
        PacketSerializer::put_actor_unique_id(stream, self.player_actor_unique_id);
    }

    fn decode(stream: &mut Reader) -> Camera {
        let camera_actor_unique_id = PacketSerializer::get_actor_unique_id(stream);
        let player_actor_unique_id = PacketSerializer::get_actor_unique_id(stream);

        Camera { camera_actor_unique_id, player_actor_unique_id }
    }
}
