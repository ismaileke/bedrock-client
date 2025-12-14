use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct BiomeMultinoiseGenRulesData {
    pub temperature: f32,
    pub humidity: f32,
    pub altitude: f32,
    pub weirdness: f32,
    pub weight: f32,
}

impl BiomeMultinoiseGenRulesData {
    pub fn new(
        temperature: f32,
        humidity: f32,
        altitude: f32,
        weirdness: f32,
        weight: f32,
    ) -> Self {
        BiomeMultinoiseGenRulesData {
            temperature,
            humidity,
            altitude,
            weirdness,
            weight,
        }
    }

    pub fn read(stream: &mut Stream) -> BiomeMultinoiseGenRulesData {
        let temperature = stream.get_f32_le();
        let humidity = stream.get_f32_le();
        let altitude = stream.get_f32_le();
        let weirdness = stream.get_f32_le();
        let weight = stream.get_f32_le();

        BiomeMultinoiseGenRulesData::new(temperature, humidity, altitude, weirdness, weight)
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_f32_le(self.temperature);
        stream.put_f32_le(self.humidity);
        stream.put_f32_le(self.altitude);
        stream.put_f32_le(self.weirdness);
        stream.put_f32_le(self.weight);
    }
}
