use binary_utils::binary::Stream;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;

pub struct RequestNetworkSettings {
    protocol_version: u32
}

pub fn new(protocol_version: u32) -> RequestNetworkSettings {
    RequestNetworkSettings{ protocol_version }
}

impl RequestNetworkSettings {
    pub fn encode(&self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(BedrockPacketType::get_byte(BedrockPacketType::RequestNetworkSettings) as u32);
        stream.put_int(self.protocol_version);

        let mut main_stream = Stream::new(Vec::new(), 0);
        main_stream.put_byte(0xfe);
        main_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        main_stream.put(stream.get_buffer());
        main_stream.get_buffer()
    }
}