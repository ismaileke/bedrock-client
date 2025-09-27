use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::packet::Packet;

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
        stream.put_unsigned_var_int(self.id() as u32);

        stream.put_var_int(self.radius);
        stream.put_byte(self.max_radius);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> RequestChunkRadius {
        let mut stream = Stream::new(bytes, 0);

        let radius = stream.get_var_int();
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
}
