use binary_utils::binary::Stream;

#[derive(Debug)]
pub struct BiomeWeightedData {
    pub biome: u16,
    pub weight: u32
}

impl BiomeWeightedData {
    pub fn new(biome: u16, weight: u32) -> Self {
        BiomeWeightedData{ biome, weight }
    }

    pub fn read(stream: &mut Stream) -> BiomeWeightedData {
        let biome = stream.get_l_short();
        let weight = stream.get_l_int();

        BiomeWeightedData::new(biome, weight)
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_l_short(self.biome);
        stream.put_l_int(self.weight);
    }
}