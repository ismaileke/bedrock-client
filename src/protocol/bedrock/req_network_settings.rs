use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct RequestNetworkSettings {
    pub protocol_version: u32,
}

impl Packet for RequestNetworkSettings {
    fn id(&self) -> u16 {
        BedrockPacketType::IDRequestNetworkSettings.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_u32_be(self.protocol_version);
    }

    fn decode(stream: &mut Reader) -> RequestNetworkSettings {
        let protocol_version = stream.get_u32_be();

        RequestNetworkSettings { protocol_version }
    }
}
