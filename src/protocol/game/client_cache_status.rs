use crate::protocol::game::bedrock_packet_ids::BedrockPacketType;
use binary_utils::binary::Stream;

pub struct ClientCacheStatus {
    enabled: bool
}

pub fn new(enabled: bool) -> ClientCacheStatus {
    ClientCacheStatus { enabled }
}

impl ClientCacheStatus {
    pub fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(BedrockPacketType::get_byte(BedrockPacketType::ClientCacheStatus) as u32);

        stream.put_bool(self.enabled);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }
}
