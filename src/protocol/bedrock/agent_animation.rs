use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

pub struct AgentAnimation {
    pub animation_type: u8,
    pub actor_runtime_id: u64
}

pub fn new(animation_type: u8, actor_runtime_id: u64) -> AgentAnimation {
    AgentAnimation { animation_type, actor_runtime_id }
}

impl Packet for AgentAnimation {
    fn id(&self) -> u16 {
        BedrockPacketType::IDAgentAnimation.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_byte(self.animation_type);
        PacketSerializer::put_actor_runtime_id(&mut stream, self.actor_runtime_id);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(bytes: Vec<u8>) -> AgentAnimation {
        let mut stream = Stream::new(bytes, 0);

        let animation_type = stream.get_byte();
        let actor_runtime_id = PacketSerializer::get_actor_runtime_id(&mut stream);

        AgentAnimation { animation_type, actor_runtime_id }
    }

    fn debug(&self) {
        println!("Animation Type: {}", self.animation_type);
        println!("Actor Runtime ID: {}", self.actor_runtime_id);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
