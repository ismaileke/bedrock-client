use binary_utils::binary::Stream;

#[derive(Debug)]
pub struct BiomeClimateData {
    pub temperature: f32,
    pub downfall: f32,
    pub snow_accumulation_min: f32,
    pub snow_accumulation_max: f32
}

impl BiomeClimateData {
    pub fn new(
        temperature: f32,
        downfall: f32,
        snow_accumulation_min: f32,
        snow_accumulation_max: f32
    ) -> Self {
        BiomeClimateData{
            temperature,
            downfall,
            snow_accumulation_min,
            snow_accumulation_max
        }
    }

    pub fn read(stream: &mut Stream) -> BiomeClimateData {
        let temperature = stream.get_f32_le();
        let downfall = stream.get_f32_le();
        let snow_accumulation_min = stream.get_f32_le();
        let snow_accumulation_max = stream.get_f32_le();

        BiomeClimateData::new(temperature, downfall, snow_accumulation_min, snow_accumulation_max)
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_f32_le(self.temperature);
        stream.put_f32_le(self.downfall);
        stream.put_f32_le(self.snow_accumulation_min);
        stream.put_f32_le(self.snow_accumulation_max);
    }
}