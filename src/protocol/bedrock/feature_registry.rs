use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::types::feature_registry_packet_entry::FeatureRegistryPacketEntry;

pub struct FeatureRegistry {
    pub entries: Vec<FeatureRegistryPacketEntry>
}

pub fn new(entries: Vec<FeatureRegistryPacketEntry>) -> FeatureRegistry {
    FeatureRegistry { entries }
}

impl Packet for FeatureRegistry {
    fn id(&self) -> u16 {
        BedrockPacketType::IDFeatureRegistry.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_var_u32(self.entries.len() as u32);
        for entry in &self.entries {
            entry.write(&mut stream);
        }

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(bytes: Vec<u8>) -> FeatureRegistry {
        let mut stream = Stream::new(bytes, 0);

        let count = stream.get_var_u32() as usize;
        let mut entries = Vec::new();
        for _ in 0..count {
            entries.push(FeatureRegistryPacketEntry::read(&mut stream));
        }

        FeatureRegistry { entries }
    }

    fn debug(&self) {
        println!("Entries: {:?}", self.entries);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
