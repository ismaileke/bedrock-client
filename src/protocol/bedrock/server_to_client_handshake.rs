use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use std::any::Any;

#[derive(serde::Serialize, Debug)]
pub struct ServerToClientHandshake {
    pub jwt: Vec<u8>,
}

impl Packet for ServerToClientHandshake {
    fn id(&self) -> u16 {
        BedrockPacketType::IDServerToClientHandshake.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_var_u32(self.jwt.len() as u32);
        stream.put(self.jwt.clone());

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> ServerToClientHandshake {
        //let jwt = PacketSerializer::get_string(stream);
        let length = stream.get_var_u32();
        let jwt = stream.get(length);

        ServerToClientHandshake { jwt }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String { serde_json::to_string(self).unwrap() }
}
