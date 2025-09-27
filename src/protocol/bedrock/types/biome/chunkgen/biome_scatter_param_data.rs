use binary_utils::binary::Stream;
use crate::protocol::bedrock::types::biome::chunkgen::biome_coordinate_data::BiomeCoordinateData;

#[derive(Debug)]
pub struct BiomeScatterParamData {
    coordinates: Vec<BiomeCoordinateData>,
    eval_order: i32,
    chance_percent_type: i32,
    chance_percent: u16,
    chance_numerator: u32,
    chance_denominator: u32,
    iterations_type: i32,
    iterations: u16
}

impl BiomeScatterParamData {
    pub fn new(
        coordinates: Vec<BiomeCoordinateData>,
        eval_order: i32,
        chance_percent_type: i32,
        chance_percent: u16,
        chance_numerator: u32,
        chance_denominator: u32,
        iterations_type: i32,
        iterations: u16
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
        let count = stream.get_unsigned_var_int();
        for _ in 0..count {
            coordinates.push(BiomeCoordinateData::read(stream));
        }
        let eval_order = stream.get_var_int();
        let chance_percent_type = stream.get_var_int();
        let chance_percent = stream.get_l_short();
        let chance_numerator = stream.get_l_int();
        let chance_denominator = stream.get_l_int();
        let iterations_type = stream.get_var_int();
        let iterations = stream.get_l_short();

        BiomeScatterParamData::new(coordinates, eval_order, chance_percent_type, chance_percent, chance_numerator, chance_denominator, iterations_type, iterations)
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_unsigned_var_int(self.coordinates.len() as u32);
        for coord in &self.coordinates {
            coord.write(stream);
        }
        stream.put_var_int(self.eval_order);
        stream.put_var_int(self.chance_percent_type);
        stream.put_l_short(self.chance_percent);
        stream.put_l_int(self.chance_numerator);
        stream.put_l_int(self.chance_denominator);
        stream.put_var_int(self.iterations_type);
        stream.put_l_short(self.iterations);
    }
}