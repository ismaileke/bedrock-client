use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct Unknown {}

impl Packet for Unknown {
    fn id(&self) -> u16 {
        BedrockPacketType::IDUnknown.get_u8()
    }

    fn encode(&mut self, _stream: &mut Writer) {
        // No Payload
    }

    fn decode(_stream: &mut Reader) -> Unknown {
        // No Payload
        Unknown {}
    }
}
