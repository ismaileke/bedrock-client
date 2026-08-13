use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct AnimateEntity {
    pub animation: String,
    pub next_state: String,
    pub stop_expression: String,
    pub stop_expression_version: i32,
    pub controller: String,
    pub blend_out_time: f32,
    pub actor_runtime_ids: Vec<u64>,
}

impl Packet for AnimateEntity {
    fn id(&self) -> u16 {
        BedrockPacketType::IDAnimateEntity.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_string(stream, self.animation.clone());
        PacketSerializer::put_string(stream, self.next_state.clone());
        PacketSerializer::put_string(stream, self.stop_expression.clone());
        stream.put_i32_le(self.stop_expression_version);
        PacketSerializer::put_string(stream, self.controller.clone());
        stream.put_f32_le(self.blend_out_time);
        stream.put_var_u32(self.actor_runtime_ids.len() as u32);
        for actor_runtime_id in self.actor_runtime_ids.iter() {
            PacketSerializer::put_actor_runtime_id(stream, *actor_runtime_id);
        }
    }

    fn decode(stream: &mut Reader) -> AnimateEntity {
        let animation = PacketSerializer::get_string(stream);
        let next_state = PacketSerializer::get_string(stream);
        let stop_expression = PacketSerializer::get_string(stream);
        let stop_expression_version = stream.get_i32_le();
        let controller = PacketSerializer::get_string(stream);
        let blend_out_time = stream.get_f32_le();
        let actor_runtime_ids_len = stream.get_var_u32();
        let mut actor_runtime_ids = Vec::new();
        for _ in 0..actor_runtime_ids_len {
            actor_runtime_ids.push(PacketSerializer::get_actor_runtime_id(stream));
        }

        AnimateEntity {
            animation,
            next_state,
            stop_expression,
            stop_expression_version,
            controller,
            blend_out_time,
            actor_runtime_ids,
        }
    }
}
