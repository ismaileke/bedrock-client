use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

pub struct SubClientLogin {
    pub connection_request_data: String
}

pub fn new(connection_request_data: String) -> SubClientLogin {
    SubClientLogin { connection_request_data }
}

impl Packet for SubClientLogin {
    fn id(&self) -> u16 {
        BedrockPacketType::IDSubClientLogin.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        PacketSerializer::put_string(&mut stream, self.connection_request_data.clone());

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> SubClientLogin {
        let mut stream = Stream::new(bytes, 0);

        let connection_request_data = PacketSerializer::get_string(&mut stream);

        SubClientLogin { connection_request_data }
    }

    fn debug(&self) {
        println!("Connection Request Data: {}", self.connection_request_data);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
