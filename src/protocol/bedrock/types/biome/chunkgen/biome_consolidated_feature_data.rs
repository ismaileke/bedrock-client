use binary_utils::binary::Stream;
use crate::protocol::bedrock::types::biome::chunkgen::biome_scatter_param_data::BiomeScatterParamData;

#[derive(serde::Serialize, Debug)]
pub struct BiomeConsolidatedFeatureData {
    pub scatter: BiomeScatterParamData,
    pub feature: i16,
    pub identifier: i16,
    pub pass: i16,
    pub use_internal: bool
}

impl BiomeConsolidatedFeatureData {
    pub fn new(scatter: BiomeScatterParamData, feature: i16, identifier: i16, pass: i16, use_internal: bool) -> Self {
        BiomeConsolidatedFeatureData{ scatter, feature, identifier, pass, use_internal }
    }

    pub fn read(stream: &mut Stream) -> BiomeConsolidatedFeatureData {
        let scatter = BiomeScatterParamData::read(stream);
        let feature = stream.get_i16_le();
        let identifier = stream.get_i16_le();
        let pass = stream.get_i16_le();
        let use_internal = stream.get_bool();

        BiomeConsolidatedFeatureData::new(scatter, feature, identifier, pass, use_internal)
    }

    pub fn write(&self, stream: &mut Stream) {
        self.scatter.write(stream);
        stream.put_i16_le(self.feature);
        stream.put_i16_le(self.identifier);
        stream.put_i16_le(self.pass);
        stream.put_bool(self.use_internal);
    }
}