use crate::protocol::bedrock::types::biome::chunkgen::biome_consolidated_feature_data::BiomeConsolidatedFeatureData;
use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct BiomeConsolidatedFeaturesData {
    pub features: Vec<BiomeConsolidatedFeatureData>,
}
impl BiomeConsolidatedFeaturesData {
    pub fn new(features: Vec<BiomeConsolidatedFeatureData>) -> Self {
        BiomeConsolidatedFeaturesData { features }
    }

    pub fn read(stream: &mut Stream) -> BiomeConsolidatedFeaturesData {
        let mut features: Vec<BiomeConsolidatedFeatureData> = Vec::new();
        let count = stream.get_var_u32();
        for _ in 0..count {
            features.push(BiomeConsolidatedFeatureData::read(stream));
        }

        BiomeConsolidatedFeaturesData::new(features)
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_var_u32(self.features.len() as u32);
        for feature in &self.features {
            feature.write(stream);
        }
    }
}
