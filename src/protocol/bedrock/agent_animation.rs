use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct AgentAnimation {
    pub animation_type: u8,
    pub actor_runtime_id: u64,
}

impl Packet for AgentAnimation {
    fn id(&self) -> u16 {
        BedrockPacketType::IDAgentAnimation.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_u8(self.animation_type);
        PacketSerializer::put_actor_runtime_id(stream, self.actor_runtime_id);
    }

    fn decode(stream: &mut Reader) -> AgentAnimation {
        let animation_type = stream.get_u8();
        let actor_runtime_id = PacketSerializer::get_actor_runtime_id(stream);

        AgentAnimation { animation_type, actor_runtime_id }
    }
}
