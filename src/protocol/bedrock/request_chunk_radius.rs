use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use binary_utils::binary::Stream;

pub struct RequestChunkRadius {
    radius: i32,
    max_radius: u8
}

pub fn new(radius: i32, max_radius: u8) -> RequestChunkRadius {
    RequestChunkRadius{ radius, max_radius }
}

impl RequestChunkRadius {
    pub fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(BedrockPacketType::get_byte(BedrockPacketType::RequestChunkRadius) as u32);

        stream.put_var_int(self.radius);
        stream.put_byte(self.max_radius);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }
}
