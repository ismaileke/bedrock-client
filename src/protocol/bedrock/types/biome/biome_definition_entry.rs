use crate::protocol::bedrock::types::biome::chunkgen::biome_definition_chunk_gen_data::BiomeDefinitionChunkGenData;

#[derive(Debug)]
pub struct BiomeDefinitionEntry {
    pub biome_name: String,
    pub id: u16,
    pub temperature: f32,
    pub downfall: f32,
    pub red_spore_density: f32,
    pub blue_spore_density: f32,
    pub ash_density: f32,
    pub white_ash_density: f32,
    pub depth: f32,
    pub scale: f32,
    pub map_water_color: u32,
    pub rain: bool,
    pub tags: Option<Vec<String>>,
    pub chunk_gen_data: Option<BiomeDefinitionChunkGenData>
}
