use std::any::Any;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;

pub struct ServerToClientHandshake {
    pub jwt: Vec<u8>,
}

impl Packet for ServerToClientHandshake {
    fn id(&self) -> u16 {
        BedrockPacketType::IDServerToClientHandshake.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        todo!()
    }

    fn decode(bytes: Vec<u8>) -> ServerToClientHandshake {
        let mut stream = Stream::new(bytes, 0);

        let length = stream.get_unsigned_var_int();
        let jwt = stream.get(length).expect("ServerToClientHandshake JWT Error");

        ServerToClientHandshake { jwt }
    }

    fn debug(&self) {
        println!("JWT: {}", String::from_utf8(self.jwt.clone()).unwrap());
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
