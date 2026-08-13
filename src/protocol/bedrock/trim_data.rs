use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::types::trim_material::TrimMaterial;
use crate::protocol::bedrock::types::trim_pattern::TrimPattern;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct TrimData {
    pub trim_patterns: Vec<TrimPattern>,
    pub trim_materials: Vec<TrimMaterial>,
}

impl Packet for TrimData {
    fn id(&self) -> u16 {
        BedrockPacketType::IDTrimData.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_var_u32(self.trim_patterns.len() as u32);
        for trim_pattern in &self.trim_patterns {
            trim_pattern.write(stream);
        }
        stream.put_var_u32(self.trim_materials.len() as u32);
        for trim_material in &self.trim_materials {
            trim_material.write(stream);
        }
    }

    fn decode(stream: &mut Reader) -> TrimData {
        let trim_patterns_count = stream.get_var_u32() as usize;
        let mut trim_patterns = Vec::new();
        for _ in 0..trim_patterns_count {
            trim_patterns.push(TrimPattern::read(stream));
        }
        let trim_materials_count = stream.get_var_u32() as usize;
        let mut trim_materials = Vec::new();
        for _ in 0..trim_materials_count {
            trim_materials.push(TrimMaterial::read(stream));
        }

        TrimData { trim_patterns, trim_materials }
    }
}
