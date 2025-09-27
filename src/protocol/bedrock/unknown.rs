use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;

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

    fn decode(_bytes: Vec<u8>) -> Unknown {
        // No Payload
        Unknown {}
    }

    fn debug(&self) {}

    fn as_any(&self) -> &dyn Any {
        self
    }
}
