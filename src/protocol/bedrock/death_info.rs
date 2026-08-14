use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct DeathInfo {
    pub message_translation_key: String,
    pub message_parameters: Vec<String>,
}

impl Packet for DeathInfo {
    fn id(&self) -> u16 {
        BedrockPacketType::IDDeathInfo.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_string(stream, &self.message_translation_key);
        stream.put_var_u32(self.message_parameters.len() as u32);
        for message_parameter in self.message_parameters.iter() {
            PacketSerializer::put_string(stream, message_parameter);
        }
    }

    fn decode(stream: &mut Reader) -> DeathInfo {
        let message_translation_key = PacketSerializer::get_string(stream);
        let message_parameters_length = stream.get_var_u32() as usize;
        let mut message_parameters = Vec::new();
        for _ in 0..message_parameters_length {
            let message_parameter = PacketSerializer::get_string(stream);
            message_parameters.push(message_parameter);
        }

        DeathInfo { message_translation_key, message_parameters }
    }
}
