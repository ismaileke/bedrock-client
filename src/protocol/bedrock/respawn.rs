use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct Respawn {
    pub position: Vec<f32>,
    pub respawn_state: u8,
    pub actor_runtime_id: u64,
}

impl Packet for Respawn {
    fn id(&self) -> u16 {
        BedrockPacketType::IDRespawn.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_vector3(stream, self.position.clone());
        stream.put_u8(self.respawn_state);
        PacketSerializer::put_actor_runtime_id(stream, self.actor_runtime_id);
    }

    fn decode(stream: &mut Reader) -> Respawn {
        let position = PacketSerializer::get_vector3(stream);
        let respawn_state = stream.get_u8();
        let actor_runtime_id = PacketSerializer::get_actor_runtime_id(stream);

        Respawn { position, respawn_state, actor_runtime_id }
    }
}

impl Respawn {
    pub const SEARCHING_FOR_SPAWN: u8 = 0;
    pub const READY_TO_SPAWN: u8 = 1;
    pub const CLIENT_READY_TO_SPAWN: u8 = 2;
}
