use binary_utils::binary::Stream;
use crate::protocol::bedrock::types::biome::chunkgen::biome_consolidated_feature_data::BiomeConsolidatedFeatureData;

#[derive(Debug)]
pub struct BiomeConsolidatedFeaturesData {
    features: Vec<BiomeConsolidatedFeatureData>
}
impl BiomeConsolidatedFeaturesData {
    pub fn new(features: Vec<BiomeConsolidatedFeatureData>) -> Self {
        BiomeConsolidatedFeaturesData{ features }
    }

    pub fn read(stream: &mut Stream) -> BiomeConsolidatedFeaturesData {
        let mut features: Vec<BiomeConsolidatedFeatureData> = Vec::new();
        let count = stream.get_unsigned_var_int();
        for _ in 0..count {
            features.push(BiomeConsolidatedFeatureData::read(stream));
        }

        BiomeConsolidatedFeaturesData::new(features)
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_unsigned_var_int(self.features.len() as u32);
        for feature in &self.features {
            feature.write(stream);
        }
    }
}