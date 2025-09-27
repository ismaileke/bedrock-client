use binary_utils::binary::Stream;

#[derive(Debug)]
pub struct BiomeClimateData {
    temperature: f32,
    downfall: f32,
    red_spore_density: f32,
    blue_spore_density: f32,
    ash_spore_density: f32,
    white_ash_spore_density: f32,
    snow_accumulation_min: f32,
    snow_accumulation_max: f32
}

impl BiomeClimateData {
    pub fn new(
        temperature: f32,
        downfall: f32,
        red_spore_density: f32,
        blue_spore_density: f32,
        ash_spore_density: f32,
        white_ash_spore_density: f32,
        snow_accumulation_min: f32,
        snow_accumulation_max: f32
    ) -> Self {
        BiomeClimateData{
            temperature,
            downfall,
            red_spore_density,
            blue_spore_density,
            ash_spore_density,
            white_ash_spore_density,
            snow_accumulation_min,
            snow_accumulation_max
        }
    }

    pub fn read(stream: &mut Stream) -> BiomeClimateData {
        let temperature = stream.get_l_float();
        let downfall = stream.get_l_float();
        let red_spore_density = stream.get_l_float();
        let blue_spore_density = stream.get_l_float();
        let ash_spore_density = stream.get_l_float();
        let white_ash_spore_density = stream.get_l_float();
        let snow_accumulation_min = stream.get_l_float();
        let snow_accumulation_max = stream.get_l_float();

        BiomeClimateData::new(temperature, downfall, red_spore_density, blue_spore_density, ash_spore_density, white_ash_spore_density, snow_accumulation_min, snow_accumulation_max)
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_l_float(self.temperature);
        stream.put_l_float(self.downfall);
        stream.put_l_float(self.red_spore_density);
        stream.put_l_float(self.blue_spore_density);
        stream.put_l_float(self.ash_spore_density);
        stream.put_l_float(self.white_ash_spore_density);
        stream.put_l_float(self.snow_accumulation_min);
        stream.put_l_float(self.snow_accumulation_max);
    }
}