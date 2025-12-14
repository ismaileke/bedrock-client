use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::Stream;
use std::any::Any;

#[derive(serde::Serialize, Debug)]
pub struct ScriptMessage {
    pub message_id: String,
    pub value: String,
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
        stream.put_var_u32(self.id() as u32);

        PacketSerializer::put_string(&mut stream, self.message_id.clone());
        PacketSerializer::put_string(&mut stream, self.value.clone());

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> ScriptMessage {
        let message_id = PacketSerializer::get_string(stream);
        let value = PacketSerializer::get_string(stream);

        ScriptMessage { message_id, value }
    }

    fn debug(&self) {
        println!("Message ID: {}", self.message_id);
        println!("Value: {}", self.value);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
