use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

pub struct ServerPlayerPostMovePosition {
    pub position: Vec<f32>
}

pub fn new(position: Vec<f32>) -> ServerPlayerPostMovePosition {
    ServerPlayerPostMovePosition { position }
}

impl Packet for ServerPlayerPostMovePosition {
    fn id(&self) -> u16 {
        BedrockPacketType::IDServerPlayerPostMovePosition.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        PacketSerializer::put_vector3(&mut stream, self.position.clone());

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> ServerPlayerPostMovePosition {
        let mut stream = Stream::new(bytes, 0);

        let position = PacketSerializer::get_vector3(&mut stream);

        ServerPlayerPostMovePosition { position }
    }

    fn debug(&self) {
        println!("Position: {:?}", self.position);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
