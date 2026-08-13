use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct ServerSettingsRequest {}

impl Packet for ServerSettingsRequest {
    fn id(&self) -> u16 {
        BedrockPacketType::IDServerSettingsRequest.get_u8()
    }

    fn encode(&mut self, _stream: &mut Writer) {
        // No payload
    }

    fn decode(_stream: &mut Reader) -> ServerSettingsRequest {
        // No payload
        ServerSettingsRequest {}
    }
}
