use binary_utils::binary::Stream;

#[derive(Debug)]
pub struct BiomeMultinoiseGenRulesData {
    temperature: f32,
    humidity: f32,
    altitude: f32,
    weirdness: f32,
    weight: f32
}

impl BiomeMultinoiseGenRulesData {
    pub fn new(temperature: f32, humidity: f32, altitude: f32, weirdness: f32, weight: f32) -> Self {
        BiomeMultinoiseGenRulesData{ temperature, humidity, altitude, weirdness, weight }
    }

    pub fn read(stream: &mut Stream) -> BiomeMultinoiseGenRulesData {
        let temperature = stream.get_l_float();
        let humidity = stream.get_l_float();
        let altitude = stream.get_l_float();
        let weirdness = stream.get_l_float();
        let weight = stream.get_l_float();

        BiomeMultinoiseGenRulesData::new(temperature, humidity, altitude, weirdness, weight)
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_l_float(self.temperature);
        stream.put_l_float(self.humidity);
        stream.put_l_float(self.altitude);
        stream.put_l_float(self.weirdness);
        stream.put_l_float(self.weight);
    }
}