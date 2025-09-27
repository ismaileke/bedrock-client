use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

pub struct UpdateClientOptions {
    pub graphics_mode: Option<u8>
}

pub fn new(graphics_mode: Option<u8>) -> UpdateClientOptions {
    UpdateClientOptions { graphics_mode }
}

impl UpdateClientOptions {
    pub const SIMPLE: u8 = 0;
    pub const FANCY: u8 = 1;
    pub const ADVANCED: u8 = 2;
    pub const RAY_TRACED: u8 = 3;
}

impl Packet for UpdateClientOptions {
    fn id(&self) -> u16 {
        BedrockPacketType::IDUpdateClientOptions.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        PacketSerializer::write_optional(&mut stream, &self.graphics_mode, |s, v| s.put_byte(*v));

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> UpdateClientOptions {
        let mut stream = Stream::new(bytes, 0);

        let graphics_mode = PacketSerializer::read_optional(&mut stream, |s| s.get_byte());

        UpdateClientOptions { graphics_mode }
    }

    fn debug(&self) {
        println!("Graphics Mode: {:?}", self.graphics_mode);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
