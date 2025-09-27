use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;

pub struct ChunkRadiusUpdated {
    pub radius: i32
}

pub fn new(radius: i32) -> ChunkRadiusUpdated {
    ChunkRadiusUpdated { radius }
}

impl Packet for ChunkRadiusUpdated {
    fn id(&self) -> u16 {
        BedrockPacketType::IDChunkRadiusUpdated.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        stream.put_var_int(self.radius);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> ChunkRadiusUpdated {
        let mut stream = Stream::new(bytes, 0);

        let radius = stream.get_var_int();

        ChunkRadiusUpdated { radius }
    }

    fn debug(&self) {
        println!("Radius: {}", self.radius);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
