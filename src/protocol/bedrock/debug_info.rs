use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

pub struct DebugInfo {
    pub actor_unique_id: i64,
    pub data: String
}

pub fn new(actor_unique_id: i64, data: String) -> DebugInfo {
    DebugInfo { actor_unique_id, data }
}

impl Packet for DebugInfo {
    fn id(&self) -> u16 {
        BedrockPacketType::IDDebugInfo.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        PacketSerializer::put_actor_unique_id(&mut stream, self.actor_unique_id);
        PacketSerializer::put_string(&mut stream, self.data.clone());

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> DebugInfo {
        let mut stream = Stream::new(bytes, 0);

        let actor_unique_id = PacketSerializer::get_actor_unique_id(&mut stream);
        let data = PacketSerializer::get_string(&mut stream);

        DebugInfo { actor_unique_id, data }
    }

    fn debug(&self) {
        println!("Actor Unique ID: {}", self.actor_unique_id);
        println!("Data: {}", self.data);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
