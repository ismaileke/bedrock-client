use std::any::Any;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;

#[derive(serde::Serialize, Debug)]
pub struct DataStoreSync {}

pub fn new() -> DataStoreSync {
    DataStoreSync {}
}

impl Packet for DataStoreSync {
    fn id(&self) -> u16 {
        BedrockPacketType::IDDataStoreSync.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        // No payload

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(_stream: &mut Stream) -> DataStoreSync {
        // No Payload
        DataStoreSync {}
    }

    fn debug(&self) {}

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
