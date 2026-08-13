use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct AgentActionEvent {
    pub request_id: String,
    pub action: u32, //see types/agent_action_type.rs
    pub response_json: String,
}

impl Packet for AgentActionEvent {
    fn id(&self) -> u16 {
        BedrockPacketType::IDAgentActionEvent.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_string(stream, self.request_id.clone());
        stream.put_u32_le(self.action);
        PacketSerializer::put_string(stream, self.response_json.clone());
    }

    fn decode(stream: &mut Reader) -> AgentActionEvent {
        let request_id = PacketSerializer::get_string(stream);
        let action = stream.get_u32_le();
        let response_json = PacketSerializer::get_string(stream);

        AgentActionEvent { request_id, action, response_json }
    }
}
