use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::types::feature_registry_packet_entry::FeatureRegistryPacketEntry;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct FeatureRegistry {
    pub entries: Vec<FeatureRegistryPacketEntry>,
}

impl Packet for FeatureRegistry {
    fn id(&self) -> u16 {
        BedrockPacketType::IDFeatureRegistry.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_var_u32(self.entries.len() as u32);
        for entry in &self.entries {
            entry.write(stream);
        }
    }

    fn decode(stream: &mut Reader) -> FeatureRegistry {
        let count = stream.get_var_u32() as usize;
        let mut entries = Vec::new();
        for _ in 0..count {
            entries.push(FeatureRegistryPacketEntry::read(stream));
        }

        FeatureRegistry { entries }
    }
}
