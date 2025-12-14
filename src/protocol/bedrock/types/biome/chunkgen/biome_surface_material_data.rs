use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct BiomeSurfaceMaterialData {
    pub top_block: u32,
    pub mid_block: u32,
    pub sea_floor_block: u32,
    pub foundation_block: u32,
    pub sea_block: u32,
    pub sea_floor_depth: i32,
}

impl BiomeSurfaceMaterialData {
    pub fn new(
        top_block: u32,
        mid_block: u32,
        sea_floor_block: u32,
        foundation_block: u32,
        sea_block: u32,
        sea_floor_depth: i32,
    ) -> Self {
        BiomeSurfaceMaterialData {
            top_block,
            mid_block,
            sea_floor_block,
            foundation_block,
            sea_block,
            sea_floor_depth,
        }
    }

    pub fn read(stream: &mut Stream) -> BiomeSurfaceMaterialData {
        let top_block = stream.get_u32_le();
        let mid_block = stream.get_u32_le();
        let sea_floor_block = stream.get_u32_le();
        let foundation_block = stream.get_u32_le();
        let sea_block = stream.get_u32_le();
        let sea_floor_depth = stream.get_i32_le();

        BiomeSurfaceMaterialData::new(
            top_block,
            mid_block,
            sea_floor_block,
            foundation_block,
            sea_block,
            sea_floor_depth,
        )
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_u32_le(self.top_block);
        stream.put_u32_le(self.mid_block);
        stream.put_u32_le(self.sea_floor_block);
        stream.put_u32_le(self.foundation_block);
        stream.put_u32_le(self.sea_block);
        stream.put_i32_le(self.sea_floor_depth);
    }
}
