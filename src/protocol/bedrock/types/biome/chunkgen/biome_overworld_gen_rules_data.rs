use binary_utils::binary::Stream;
use crate::protocol::bedrock::types::biome::chunkgen::biome_conditional_transformation_data::BiomeConditionalTransformationData;
use crate::protocol::bedrock::types::biome::chunkgen::biome_weighted_data::BiomeWeightedData;
use crate::protocol::bedrock::types::biome::chunkgen::biome_weighted_temperature_data::BiomeWeightedTemperatureData;

#[derive(Debug)]
pub struct BiomeOverworldGenRulesData {
    pub hill_transformations: Vec<BiomeWeightedData>,
    pub mutate_transformations: Vec<BiomeWeightedData>,
    pub river_transformations: Vec<BiomeWeightedData>,
    pub shore_transformations: Vec<BiomeWeightedData>,
    pub pre_hills_edges: Vec<BiomeConditionalTransformationData>,
    pub post_shore_edges: Vec<BiomeConditionalTransformationData>,
    pub climates: Vec<BiomeWeightedTemperatureData>
}

impl BiomeOverworldGenRulesData {
    pub fn new(
        hill_transformations: Vec<BiomeWeightedData>,
        mutate_transformations: Vec<BiomeWeightedData>,
        river_transformations: Vec<BiomeWeightedData>,
        shore_transformations: Vec<BiomeWeightedData>,
        pre_hills_edges: Vec<BiomeConditionalTransformationData>,
        post_shore_edges: Vec<BiomeConditionalTransformationData>,
        climates: Vec<BiomeWeightedTemperatureData>
    ) -> Self {
        BiomeOverworldGenRulesData{
            hill_transformations,
            mutate_transformations,
            river_transformations,
            shore_transformations,
            pre_hills_edges,
            post_shore_edges,
            climates
        }
    }

    pub fn read(stream: &mut Stream) -> BiomeOverworldGenRulesData {
        let mut hill_transformations = Vec::new();
        let mut count = stream.get_var_u32();
        for _ in 0..count {
            hill_transformations.push(BiomeWeightedData::read(stream));
        }
        let mut mutate_transformations = Vec::new();
        count = stream.get_var_u32();
        for _ in 0..count {
            mutate_transformations.push(BiomeWeightedData::read(stream));
        }
        let mut river_transformations = Vec::new();
        count = stream.get_var_u32();
        for _ in 0..count {
            river_transformations.push(BiomeWeightedData::read(stream));
        }
        let mut shore_transformations = Vec::new();
        count = stream.get_var_u32();
        for _ in 0..count {
            shore_transformations.push(BiomeWeightedData::read(stream));
        }
        let mut pre_hills_edges = Vec::new();
        count = stream.get_var_u32();
        for _ in 0..count {
            pre_hills_edges.push(BiomeConditionalTransformationData::read(stream));
        }
        let mut post_shore_edges = Vec::new();
        count = stream.get_var_u32();
        for _ in 0..count {
            post_shore_edges.push(BiomeConditionalTransformationData::read(stream));
        }
        let mut climates = Vec::new();
        count = stream.get_var_u32();
        for _ in 0..count {
            climates.push(BiomeWeightedTemperatureData::read(stream));
        }

        BiomeOverworldGenRulesData::new(hill_transformations, mutate_transformations, river_transformations, shore_transformations, pre_hills_edges, post_shore_edges, climates)
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_var_u32(self.hill_transformations.len() as u32);
        for hill_transformation in &self.hill_transformations {
            hill_transformation.write(stream);
        }
        stream.put_var_u32(self.mutate_transformations.len() as u32);
        for mutate_transformation in &self.mutate_transformations {
            mutate_transformation.write(stream);
        }
        stream.put_var_u32(self.river_transformations.len() as u32);
        for river_transformation in &self.river_transformations {
            river_transformation.write(stream);
        }
        stream.put_var_u32(self.shore_transformations.len() as u32);
        for shore_transformation in &self.shore_transformations {
            shore_transformation.write(stream);
        }
        stream.put_var_u32(self.pre_hills_edges.len() as u32);
        for pre_hill_transformation in &self.pre_hills_edges {
            pre_hill_transformation.write(stream);
        }
        stream.put_var_u32(self.post_shore_edges.len() as u32);
        for post_shore_transformation in &self.post_shore_edges {
            post_shore_transformation.write(stream);
        }
        stream.put_var_u32(self.climates.len() as u32);
        for climate_transformation in &self.climates {
            climate_transformation.write(stream);
        }
    }
}