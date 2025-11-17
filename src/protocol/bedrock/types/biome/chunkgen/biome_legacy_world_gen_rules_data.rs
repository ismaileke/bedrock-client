use binary_utils::binary::Stream;
use crate::protocol::bedrock::types::biome::chunkgen::biome_conditional_transformation_data::BiomeConditionalTransformationData;

#[derive(Debug)]
pub struct BiomeLegacyWorldGenRulesData {
    pub legacy_pre_hills: Vec<BiomeConditionalTransformationData>
}

impl BiomeLegacyWorldGenRulesData {
    pub fn new(legacy_pre_hills: Vec<BiomeConditionalTransformationData>) -> Self {
        BiomeLegacyWorldGenRulesData{ legacy_pre_hills }
    }

    pub fn read(stream: &mut Stream) -> BiomeLegacyWorldGenRulesData {
        let mut legacy_pre_hills = Vec::new();
        let count = stream.get_unsigned_var_int();
        for _ in 0..count {
            legacy_pre_hills.push(BiomeConditionalTransformationData::read(stream));
        }

        BiomeLegacyWorldGenRulesData::new(legacy_pre_hills)
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_unsigned_var_int(self.legacy_pre_hills.len() as u32);
        for legacy_pre_hill in &self.legacy_pre_hills {
            legacy_pre_hill.write(stream);
        }
    }
}