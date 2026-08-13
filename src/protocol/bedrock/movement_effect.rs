use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct MovementEffect {
    pub actor_runtime_id: u64,
    pub effect_type: u32, //see types/movement_effect_type.rs
    pub duration: u32,
    pub tick: u64,
}

impl Packet for MovementEffect {
    fn id(&self) -> u16 {
        BedrockPacketType::IDMovementEffect.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_actor_runtime_id(stream, self.actor_runtime_id);
        stream.put_var_u32(self.effect_type);
        stream.put_var_u32(self.duration);
        stream.put_var_u64(self.tick);
    }

    fn decode(stream: &mut Reader) -> MovementEffect {
        let actor_runtime_id = PacketSerializer::get_actor_runtime_id(stream);
        let effect_type = stream.get_var_u32();
        let duration = stream.get_var_u32();
        let tick = stream.get_var_u64();

        MovementEffect { actor_runtime_id, effect_type, duration, tick }
    }
}
