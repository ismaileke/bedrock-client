use binary_utils::binary::Stream;
use crate::protocol::bedrock::types::biome::chunkgen::biome_weighted_data::BiomeWeightedData;

#[derive(Debug)]
pub struct BiomeConditionalTransformationData {
    pub weighted_biomes: Vec<BiomeWeightedData>,
    pub condition_json: i16,
    pub min_passing_neighbors: u32
}
impl BiomeConditionalTransformationData {
    pub fn new(weighted_biomes: Vec<BiomeWeightedData>, condition_json: i16, min_passing_neighbors: u32) -> Self {
        BiomeConditionalTransformationData{ weighted_biomes, condition_json, min_passing_neighbors }
    }

    pub fn read(stream: &mut Stream) -> BiomeConditionalTransformationData {
        let mut weighted_biomes = Vec::new();
        let count = stream.get_var_u32();
        for _ in 0..count {
            weighted_biomes.push(BiomeWeightedData::read(stream));
        }
        let condition_json = stream.get_i16_le();
        let min_passing_neighbors = stream.get_u32_le();

        BiomeConditionalTransformationData::new(weighted_biomes, condition_json, min_passing_neighbors)
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_var_u32(self.weighted_biomes.len() as u32);
        for biome in &self.weighted_biomes {
            biome.write(stream);
        }
        stream.put_i16_le(self.condition_json);
        stream.put_u32_le(self.min_passing_neighbors);
    }
}