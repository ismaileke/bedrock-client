use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::types::feature_registry_packet_entry::FeatureRegistryPacketEntry;
use binary_utils::binary::Stream;
use std::any::Any;

#[derive(serde::Serialize, Debug)]
pub struct FeatureRegistry {
    pub entries: Vec<FeatureRegistryPacketEntry>,
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

    fn decode(stream: &mut Stream) -> FeatureRegistry {
        let count = stream.get_var_u32() as usize;
        let mut entries = Vec::new();
        for _ in 0..count {
            entries.push(FeatureRegistryPacketEntry::read(stream));
        }

        FeatureRegistry { entries }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String { serde_json::to_string(self).unwrap() }
}
