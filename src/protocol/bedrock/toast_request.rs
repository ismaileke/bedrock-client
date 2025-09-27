use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

pub struct ToastRequest {
    pub title: String,
    pub body: String
}

pub fn new(title: String, body: String) -> ToastRequest {
    ToastRequest { title, body }
}

impl Packet for ToastRequest {
    fn id(&self) -> u16 {
        BedrockPacketType::IDToastRequest.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        PacketSerializer::put_string(&mut stream, self.title.clone());
        PacketSerializer::put_string(&mut stream, self.body.clone());

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> ToastRequest {
        let mut stream = Stream::new(bytes, 0);

        let title = PacketSerializer::get_string(&mut stream);
        let body = PacketSerializer::get_string(&mut stream);

        ToastRequest { title, body }
    }

    fn debug(&self) {
        println!("Title: {}", self.title);
        println!("Body: {}", self.body);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
