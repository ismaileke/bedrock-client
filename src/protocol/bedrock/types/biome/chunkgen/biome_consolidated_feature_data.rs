use binary_utils::binary::Stream;
use crate::protocol::bedrock::types::biome::chunkgen::biome_scatter_param_data::BiomeScatterParamData;

#[derive(Debug)]
pub struct BiomeConsolidatedFeatureData {
    scatter: BiomeScatterParamData,
    feature: u16,
    identifier: u16,
    pass: u16,
    use_internal: bool
}

impl BiomeConsolidatedFeatureData {
    pub fn new(scatter: BiomeScatterParamData, feature: u16, identifier: u16, pass: u16, use_internal: bool) -> Self {
        BiomeConsolidatedFeatureData{ scatter, feature, identifier, pass, use_internal }
    }

    pub fn read(stream: &mut Stream) -> BiomeConsolidatedFeatureData {
        let scatter = BiomeScatterParamData::read(stream);
        let feature = stream.get_l_short();
        let identifier = stream.get_l_short();
        let pass = stream.get_l_short();
        let use_internal = stream.get_bool();

        BiomeConsolidatedFeatureData::new(scatter, feature, identifier, pass, use_internal)
    }

    pub fn write(&self, stream: &mut Stream) {
        self.scatter.write(stream);
        stream.put_l_short(self.feature);
        stream.put_l_short(self.identifier);
        stream.put_l_short(self.pass);
        stream.put_bool(self.use_internal);
    }
}