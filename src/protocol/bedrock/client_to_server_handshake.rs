use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use binary_utils::binary::Stream;

pub struct ClientToServerHandshake {
}

pub fn new() -> ClientToServerHandshake {
    ClientToServerHandshake{}
}

impl ClientToServerHandshake {
    pub fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(BedrockPacketType::get_byte(BedrockPacketType::ClientToServerHandshake) as u32);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    pub fn debug(&self) {
    }
}