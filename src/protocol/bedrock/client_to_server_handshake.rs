use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::packet::Packet;

pub struct ClientToServerHandshake {}

pub fn new() -> ClientToServerHandshake {
    ClientToServerHandshake{}
}

impl Packet for ClientToServerHandshake {
    fn id(&self) -> u16 {
        BedrockPacketType::IDClientToServerHandshake.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(_stream: &mut Stream) -> ClientToServerHandshake {
        ClientToServerHandshake{}
    }

    fn debug(&self) {
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
