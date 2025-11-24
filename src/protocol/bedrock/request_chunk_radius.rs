use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::packet::Packet;

#[derive(serde::Serialize, Debug)]
pub struct RequestChunkRadius {
    pub radius: i32,
    pub max_radius: u8
}

pub fn new(radius: i32, max_radius: u8) -> RequestChunkRadius {
    RequestChunkRadius{ radius, max_radius }
}

impl Packet for RequestChunkRadius {
    fn id(&self) -> u16 {
        BedrockPacketType::IDRequestChunkRadius.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_var_i32(self.radius);
        stream.put_byte(self.max_radius);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> RequestChunkRadius {
        let radius = stream.get_var_i32();
        let max_radius = stream.get_byte();

        RequestChunkRadius { radius, max_radius }
    }


    fn debug(&self) {
        println!("Radius: {}", self.radius);
        println!("Maximum Radius: {}", self.max_radius);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
