use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::Stream;
use std::any::Any;

#[derive(serde::Serialize, Debug)]
pub struct DeathInfo {
    pub message_translation_key: String,
    pub message_parameters: Vec<String>,
}

pub fn new(message_translation_key: String, message_parameters: Vec<String>) -> DeathInfo {
    DeathInfo {
        message_translation_key,
        message_parameters,
    }
}

impl Packet for DeathInfo {
    fn id(&self) -> u16 {
        BedrockPacketType::IDDeathInfo.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        PacketSerializer::put_string(&mut stream, self.message_translation_key.clone());
        stream.put_var_u32(self.message_parameters.len() as u32);
        for message_parameter in self.message_parameters.iter() {
            PacketSerializer::put_string(&mut stream, message_parameter.clone());
        }

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> DeathInfo {
        let message_translation_key = PacketSerializer::get_string(stream);
        let message_parameters_length = stream.get_var_u32() as usize;
        let mut message_parameters = Vec::new();
        for _ in 0..message_parameters_length {
            let message_parameter = PacketSerializer::get_string(stream);
            message_parameters.push(message_parameter);
        }

        DeathInfo {
            message_translation_key,
            message_parameters,
        }
    }

    fn debug(&self) {
        println!("Message Translation Key: {}", self.message_translation_key);
        println!("Message Parameters: {:?}", self.message_parameters);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
