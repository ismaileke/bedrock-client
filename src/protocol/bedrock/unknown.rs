use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use std::any::Any;

#[derive(serde::Serialize, Debug)]
pub struct Unknown {}

pub fn new() -> Unknown {
    Unknown {}
}

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

    fn debug(&self) {}

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
