use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::types::trim_material::TrimMaterial;
use crate::protocol::bedrock::types::trim_pattern::TrimPattern;

pub struct TrimData {
    pub trim_patterns: Vec<TrimPattern>,
    pub trim_materials: Vec<TrimMaterial>
}

pub fn new(trim_patterns: Vec<TrimPattern>, trim_materials: Vec<TrimMaterial>) -> TrimData {
    TrimData { trim_patterns, trim_materials }
}

impl Packet for TrimData {
    fn id(&self) -> u16 {
        BedrockPacketType::IDTrimData.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        stream.put_unsigned_var_int(self.trim_patterns.len() as u32);
        for trim_pattern in &self.trim_patterns {
            trim_pattern.write(&mut stream);
        }
        stream.put_unsigned_var_int(self.trim_materials.len() as u32);
        for trim_material in &self.trim_materials {
            trim_material.write(&mut stream);
        }

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> TrimData {
        let mut stream = Stream::new(bytes, 0);

        let trim_patterns_count = stream.get_unsigned_var_int() as usize;
        let mut trim_patterns = Vec::new();
        for _ in 0..trim_patterns_count {
            trim_patterns.push(TrimPattern::read(&mut stream));
        }
        let trim_materials_count = stream.get_unsigned_var_int() as usize;
        let mut trim_materials = Vec::new();
        for _ in 0..trim_materials_count {
            trim_materials.push(TrimMaterial::read(&mut stream));
        }

        TrimData { trim_patterns, trim_materials }
    }

    fn debug(&self) {
        println!("Trim Patterns: {:?}", self.trim_patterns);
        println!("Trim Materials: {:?}", self.trim_materials);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
