use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct BiomeCoordinateData {
    pub min_value_type: i32,
    pub min_value: i16,
    pub max_value_type: i32,
    pub max_value: i16,
    pub grid_offset: u32,
    pub grid_step_size: u32,
    pub distribution: i32,
}

impl BiomeCoordinateData {
    pub fn new(
        min_value_type: i32,
        min_value: i16,
        max_value_type: i32,
        max_value: i16,
        grid_offset: u32,
        grid_step_size: u32,
        distribution: i32,
    ) -> Self {
        BiomeCoordinateData {
            min_value_type,
            min_value,
            max_value_type,
            max_value,
            grid_offset,
            grid_step_size,
            distribution,
        }
    }

    pub fn read(stream: &mut Stream) -> BiomeCoordinateData {
        let min_value_type = stream.get_var_i32();
        let min_value = stream.get_i16_le();
        let max_value_type = stream.get_var_i32();
        let max_value = stream.get_i16_le();
        let grid_offset = stream.get_u32_le();
        let grid_step_size = stream.get_u32_le();
        let distribution = stream.get_var_i32();

        BiomeCoordinateData::new(
            min_value_type,
            min_value,
            max_value_type,
            max_value,
            grid_offset,
            grid_step_size,
            distribution,
        )
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_var_i32(self.min_value_type);
        stream.put_i16_le(self.min_value);
        stream.put_var_i32(self.max_value_type);
        stream.put_i16_le(self.max_value);
        stream.put_u32_le(self.grid_offset);
        stream.put_u32_le(self.grid_step_size);
        stream.put_var_i32(self.distribution);
    }
}
