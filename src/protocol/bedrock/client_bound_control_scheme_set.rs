use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;

pub struct ClientBoundControlSchemeSet {
    pub scheme: u8 //see types/control_scheme.rs
}

pub fn new(scheme: u8) -> ClientBoundControlSchemeSet {
    ClientBoundControlSchemeSet { scheme }
}

impl Packet for ClientBoundControlSchemeSet {
    fn id(&self) -> u16 {
        BedrockPacketType::IDClientBoundControlSchemeSet.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        stream.put_byte(self.scheme);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> ClientBoundControlSchemeSet {
        let mut stream = Stream::new(bytes, 0);

        let scheme = stream.get_byte();

        ClientBoundControlSchemeSet { scheme }
    }

    fn debug(&self) {
        println!("Scheme: {}", self.scheme);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
