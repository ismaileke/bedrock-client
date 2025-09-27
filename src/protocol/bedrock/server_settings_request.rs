use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;

pub struct ServerSettingsRequest {}

pub fn new() -> ServerSettingsRequest {
    ServerSettingsRequest {  }
}

impl Packet for ServerSettingsRequest {
    fn id(&self) -> u16 {
        BedrockPacketType::IDServerSettingsRequest.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        // No payload

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(_bytes: Vec<u8>) -> ServerSettingsRequest {
        // No payload
        ServerSettingsRequest {}
    }

    fn debug(&self) {
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
