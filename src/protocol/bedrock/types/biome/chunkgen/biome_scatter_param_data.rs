use binary_utils::binary::Stream;
use crate::protocol::bedrock::types::biome::chunkgen::biome_coordinate_data::BiomeCoordinateData;

#[derive(serde::Serialize, Debug)]
pub struct BiomeScatterParamData {
    pub coordinates: Vec<BiomeCoordinateData>,
    pub eval_order: i32,
    pub chance_percent_type: i32,
    pub chance_percent: i16,
    pub chance_numerator: i32,
    pub chance_denominator: i32,
    pub iterations_type: i32,
    pub iterations: i16
}

impl BiomeScatterParamData {
    pub fn new(
        coordinates: Vec<BiomeCoordinateData>,
        eval_order: i32,
        chance_percent_type: i32,
        chance_percent: i16,
        chance_numerator: i32,
        chance_denominator: i32,
        iterations_type: i32,
        iterations: i16
    ) -> Self {
        BiomeScatterParamData {
            coordinates,
            eval_order,
            chance_percent_type,
            chance_percent,
            chance_numerator,
            chance_denominator,
            iterations_type,
            iterations
        }
    }

    pub fn read(stream: &mut Stream) -> BiomeScatterParamData {
        let mut coordinates = Vec::<BiomeCoordinateData>::new();
        let count = stream.get_var_u32();
        for _ in 0..count {
            coordinates.push(BiomeCoordinateData::read(stream));
        }
        let eval_order = stream.get_var_i32();
        let chance_percent_type = stream.get_var_i32();
        let chance_percent = stream.get_i16_le();
        let chance_numerator = stream.get_i32_le();
        let chance_denominator = stream.get_i32_le();
        let iterations_type = stream.get_var_i32();
        let iterations = stream.get_i16_le();

        BiomeScatterParamData::new(coordinates, eval_order, chance_percent_type, chance_percent, chance_numerator, chance_denominator, iterations_type, iterations)
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_var_u32(self.coordinates.len() as u32);
        for coord in &self.coordinates {
            coord.write(stream);
        }
        stream.put_var_i32(self.eval_order);
        stream.put_var_i32(self.chance_percent_type);
        stream.put_i16_le(self.chance_percent);
        stream.put_i32_le(self.chance_numerator);
        stream.put_i32_le(self.chance_denominator);
        stream.put_var_i32(self.iterations_type);
        stream.put_i16_le(self.iterations);
    }
}