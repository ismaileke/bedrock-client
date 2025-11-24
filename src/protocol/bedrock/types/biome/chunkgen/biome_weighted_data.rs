use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct BiomeWeightedData {
    pub biome: i16,
    pub weight: u32
}

impl BiomeWeightedData {
    pub fn new(biome: i16, weight: u32) -> Self {
        BiomeWeightedData{ biome, weight }
    }

    pub fn read(stream: &mut Stream) -> BiomeWeightedData {
        let biome = stream.get_i16_le();
        let weight = stream.get_u32_le();

        BiomeWeightedData::new(biome, weight)
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_i16_le(self.biome);
        stream.put_u32_le(self.weight);
    }
}