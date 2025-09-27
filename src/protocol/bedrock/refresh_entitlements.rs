use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;

pub struct RefreshEntitlements {}

pub fn new() -> RefreshEntitlements {
    RefreshEntitlements {}
}

impl Packet for RefreshEntitlements {
    fn id(&self) -> u16 {
        BedrockPacketType::IDRefreshEntitlements.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        // No Payload

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(_bytes: Vec<u8>) -> RefreshEntitlements {
        // No Payload
        RefreshEntitlements {}
    }

    fn debug(&self) {}

    fn as_any(&self) -> &dyn Any {
        self
    }
}
