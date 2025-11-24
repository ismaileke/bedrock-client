use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct BiomeMountainParamsData {
    pub steep_block: u32,
    pub north_slopes: bool,
    pub south_slopes: bool,
    pub west_slopes: bool,
    pub east_slopes: bool,
    pub top_slide_enabled: bool
}

impl BiomeMountainParamsData {
    pub fn new(
        steep_block: u32,
        north_slopes: bool,
        south_slopes: bool,
        west_slopes: bool,
        east_slopes: bool,
        top_slide_enabled: bool
    ) -> Self {
        BiomeMountainParamsData{
            steep_block,
            north_slopes,
            south_slopes,
            west_slopes,
            east_slopes,
            top_slide_enabled
        }
    }

    pub fn read(stream: &mut Stream) -> BiomeMountainParamsData {
        let steep_block = stream.get_u32_le();
        let north_slopes = stream.get_bool();
        let south_slopes = stream.get_bool();
        let west_slopes = stream.get_bool();
        let east_slopes = stream.get_bool();
        let top_slide_enabled = stream.get_bool();

        BiomeMountainParamsData::new(steep_block, north_slopes, south_slopes, west_slopes, east_slopes, top_slide_enabled)
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_u32_le(self.steep_block);
        stream.put_bool(self.north_slopes);
        stream.put_bool(self.south_slopes);
        stream.put_bool(self.west_slopes);
        stream.put_bool(self.east_slopes);
        stream.put_bool(self.top_slide_enabled);
    }
}