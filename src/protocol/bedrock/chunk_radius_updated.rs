use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
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
        stream.put_var_u32(self.id() as u32);

        stream.put_var_i32(self.radius);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> ChunkRadiusUpdated {
        let radius = stream.get_var_i32();

        ChunkRadiusUpdated { radius }
    }

    fn debug(&self) {
        println!("Radius: {}", self.radius);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
