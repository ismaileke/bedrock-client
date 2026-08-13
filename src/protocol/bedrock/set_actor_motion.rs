use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct SetActorMotion {
    pub actor_runtime_id: u64,
    pub motion: Vec<f32>,
    pub tick: u64,
}

impl Packet for SetActorMotion {
    fn id(&self) -> u16 {
        BedrockPacketType::IDSetActorMotion.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_actor_runtime_id(stream, self.actor_runtime_id);
        PacketSerializer::put_vector3(stream, self.motion.clone());
        stream.put_var_u64(self.tick);
    }

    fn decode(stream: &mut Reader) -> SetActorMotion {
        let actor_runtime_id = PacketSerializer::get_actor_runtime_id(stream);
        let motion = PacketSerializer::get_vector3(stream);
        let tick = stream.get_var_u64();

        SetActorMotion { actor_runtime_id, motion, tick }
    }
}
