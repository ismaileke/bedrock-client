use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct UpdateClientOptions {
    pub graphics_mode: Option<u8>,
    pub filter_profanity_change: Option<bool>,
}

impl Packet for UpdateClientOptions {
    fn id(&self) -> u16 {
        BedrockPacketType::IDUpdateClientOptions.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::write_optional(stream, &self.graphics_mode, |s, v| s.put_u8(*v));
        PacketSerializer::write_optional(stream, &self.filter_profanity_change, |s, v| s.put_bool(*v));
    }

    fn decode(stream: &mut Reader) -> UpdateClientOptions {
        let graphics_mode = PacketSerializer::read_optional(stream, |s| s.get_u8());
        let filter_profanity_change = PacketSerializer::read_optional(stream, |s| s.get_bool());

        UpdateClientOptions { graphics_mode, filter_profanity_change }
    }
}

impl UpdateClientOptions {
    pub const SIMPLE: u8 = 0;
    pub const FANCY: u8 = 1;
    pub const ADVANCED: u8 = 2;
    pub const RAY_TRACED: u8 = 3;
}
