use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

pub struct ScriptMessage {
    pub message_id: String,
    pub value: String
}

pub fn new(message_id: String, value: String) -> ScriptMessage {
    ScriptMessage { message_id, value }
}

impl Packet for ScriptMessage {
    fn id(&self) -> u16 {
        BedrockPacketType::IDScriptMessage.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        PacketSerializer::put_string(&mut stream, self.message_id.clone());
        PacketSerializer::put_string(&mut stream, self.value.clone());

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> ScriptMessage {
        let mut stream = Stream::new(bytes, 0);

        let message_id = PacketSerializer::get_string(&mut stream);
        let value = PacketSerializer::get_string(&mut stream);

        ScriptMessage { message_id, value }
    }

    fn debug(&self) {
        println!("Message ID: {}", self.message_id);
        println!("Value: {}", self.value);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
