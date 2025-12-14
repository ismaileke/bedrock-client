use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use std::any::Any;

#[derive(serde::Serialize, Debug)]
pub struct ClientCacheStatus {
    pub enabled: bool,
}

pub fn new(enabled: bool) -> ClientCacheStatus {
    ClientCacheStatus { enabled }
}

impl Packet for ClientCacheStatus {
    fn id(&self) -> u16 {
        BedrockPacketType::IDClientCacheStatus.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_bool(self.enabled);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> ClientCacheStatus {
        let enabled = stream.get_bool();
        ClientCacheStatus { enabled }
    }

    fn debug(&self) {
        println!("Enabled: {}", self.enabled);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
