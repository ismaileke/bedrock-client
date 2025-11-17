use binary_utils::binary::Stream;

#[derive(Debug)]
pub struct BiomeWeightedTemperatureData {
    pub temperature: i32,
    pub weight: u32
}

impl BiomeWeightedTemperatureData {
    pub fn new(temperature: i32, weight: u32) -> Self {
        BiomeWeightedTemperatureData{ temperature , weight }
    }

    pub fn read(stream: &mut Stream) -> BiomeWeightedTemperatureData {
        let temperature  = stream.get_var_int();
        let weight = stream.get_l_int();

        BiomeWeightedTemperatureData::new(temperature, weight)
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_var_int(self.temperature);
        stream.put_l_int(self.weight);
    }
}