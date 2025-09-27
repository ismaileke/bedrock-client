use binary_utils::binary::Stream;

#[derive(Debug)]
pub struct BiomeCoordinateData {
    min_value_type: i32,
    min_value: u16,
    max_value_type: i32,
    max_value: u16,
    grid_offset: u32,
    grid_step_size: u32,
    distribution: i32
}

impl BiomeCoordinateData {
    pub fn new(
        min_value_type: i32,
        min_value: u16,
        max_value_type: i32,
        max_value: u16,
        grid_offset: u32,
        grid_step_size: u32,
        distribution: i32
    ) -> Self {
        BiomeCoordinateData{
            min_value_type,
            min_value,
            max_value_type,
            max_value,
            grid_offset,
            grid_step_size,
            distribution
        }
    }

    pub fn read(stream: &mut Stream) -> BiomeCoordinateData {
        let min_value_type = stream.get_var_int();
        let min_value = stream.get_l_short();
        let max_value_type = stream.get_var_int();
        let max_value = stream.get_l_short();
        let grid_offset = stream.get_l_int();
        let grid_step_size = stream.get_l_int();
        let distribution = stream.get_var_int();

        BiomeCoordinateData::new(min_value_type, min_value, max_value_type, max_value, grid_offset, grid_step_size, distribution)
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_var_int(self.min_value_type);
        stream.put_l_short(self.min_value);
        stream.put_var_int(self.max_value_type);
        stream.put_l_short(self.max_value);
        stream.put_l_int(self.grid_offset);
        stream.put_l_int(self.grid_step_size);
        stream.put_var_int(self.distribution);
    }
}