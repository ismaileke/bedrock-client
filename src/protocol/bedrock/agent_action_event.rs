use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

pub struct AgentActionEvent {
    pub request_id: String,
    pub action: u32, //see types/agent_action_type.rs
    pub response_json: String
}

pub fn new(request_id: String, action: u32, response_json: String) -> AgentActionEvent {
    AgentActionEvent { request_id, action, response_json }
}

impl Packet for AgentActionEvent {
    fn id(&self) -> u16 {
        BedrockPacketType::IDAgentActionEvent.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        PacketSerializer::put_string(&mut stream, self.request_id.clone());
        stream.put_u32_le(self.action);
        PacketSerializer::put_string(&mut stream, self.response_json.clone());

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(bytes: Vec<u8>) -> AgentActionEvent {
        let mut stream = Stream::new(bytes, 0);

        let request_id = PacketSerializer::get_string(&mut stream);
        let action = stream.get_u32_le();
        let response_json = PacketSerializer::get_string(&mut stream);

        AgentActionEvent { request_id, action, response_json }
    }

    fn debug(&self) {
        println!("Request ID: {}", self.request_id);
        println!("Action: {}", self.action);
        println!("Response JSON: {}", self.response_json);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
