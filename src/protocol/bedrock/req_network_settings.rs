use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use std::any::Any;
use crate::protocol::raknet::packet_ids::PacketType;

#[derive(serde::Serialize, Debug)]
pub struct RequestNetworkSettings {
    pub protocol_version: u32,
}

impl Packet for RequestNetworkSettings {
    fn id(&self) -> u16 {
        BedrockPacketType::IDRequestNetworkSettings.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_u32_be(self.protocol_version);

        let mut main_stream = Stream::new(Vec::new(), 0);
        main_stream.put_byte(PacketType::Game.get_byte());
        main_stream.put_var_u32(stream.get_buffer().len() as u32);
        main_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(main_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> RequestNetworkSettings {
        let protocol_version = stream.get_u32_be();

        RequestNetworkSettings { protocol_version }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String { serde_json::to_string(self).unwrap() }
}
