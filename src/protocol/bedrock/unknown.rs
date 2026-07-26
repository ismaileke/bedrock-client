use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct Unknown {}

impl Packet for Unknown {
    fn id(&self) -> u16 {
        BedrockPacketType::IDUnknown.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        vec![]
    }

    fn decode(_stream: &mut Stream) -> Unknown {
        // No Payload
        Unknown {}
    }
}
