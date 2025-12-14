use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct BiomeReplacementData {
    pub biome: i16,
    pub dimension: i32,
    pub target_biomes: Vec<i16>,
    pub amount: f32,
    pub replacement_index: u32,
}

impl BiomeReplacementData {
    pub fn new(
        biome: i16,
        dimension: i32,
        target_biomes: Vec<i16>,
        amount: f32,
        replacement_index: u32,
    ) -> Self {
        BiomeReplacementData {
            biome,
            dimension,
            target_biomes,
            amount,
            replacement_index,
        }
    }

    pub fn read(stream: &mut Stream) -> BiomeReplacementData {
        let biome = stream.get_i16_le();
        let dimension = stream.get_var_i32();

        let mut target_biomes = Vec::new();
        let target_biome_count = stream.get_var_u32();
        for _ in 0..target_biome_count {
            target_biomes.push(stream.get_i16_le());
        }
        let amount = stream.get_f32_le();
        let replacement_index = stream.get_u32_le();

        BiomeReplacementData::new(biome, dimension, target_biomes, amount, replacement_index)
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_i16_le(self.biome);
        stream.put_var_i32(self.dimension);

        stream.put_var_u32(self.target_biomes.len() as u32);
        for target_biome in &self.target_biomes {
            stream.put_i16_le(*target_biome);
        }
        stream.put_f32_le(self.amount);
        stream.put_u32_le(self.replacement_index);
    }
}
