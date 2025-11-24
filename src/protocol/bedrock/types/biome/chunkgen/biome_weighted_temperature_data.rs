use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct BiomeWeightedTemperatureData {
    pub temperature: i32,
    pub weight: u32
}

impl BiomeWeightedTemperatureData {
    pub fn new(temperature: i32, weight: u32) -> Self {
        BiomeWeightedTemperatureData{ temperature , weight }
    }

    pub fn read(stream: &mut Stream) -> BiomeWeightedTemperatureData {
        let temperature  = stream.get_var_i32();
        let weight = stream.get_u32_le();

        BiomeWeightedTemperatureData::new(temperature, weight)
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_var_i32(self.temperature);
        stream.put_u32_le(self.weight);
    }
}