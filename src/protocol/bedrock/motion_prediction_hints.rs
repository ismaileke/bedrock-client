use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct MotionPredictionHints {
    pub actor_runtime_id: u64,
    pub motion: Vec<f32>,
    pub on_ground: bool,
}

impl Packet for MotionPredictionHints {
    fn id(&self) -> u16 {
        BedrockPacketType::IDMotionPredictionHints.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_actor_runtime_id(stream, self.actor_runtime_id);
        PacketSerializer::put_vector3(stream, self.motion.clone());
        stream.put_bool(self.on_ground);
    }

    fn decode(stream: &mut Reader) -> MotionPredictionHints {
        let actor_runtime_id = PacketSerializer::get_actor_runtime_id(stream);
        let motion = PacketSerializer::get_vector3(stream);
        let on_ground = stream.get_bool();

        MotionPredictionHints { actor_runtime_id, motion, on_ground }
    }
}
