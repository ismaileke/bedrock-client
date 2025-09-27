use binary_utils::binary::Stream;

#[derive(Debug)]
pub struct BiomeSurfaceMaterialData {
    top_block: u32,
    mid_block: u32,
    sea_floor_block: u32,
    foundation_block: u32,
    sea_block: u32,
    sea_floor_depth: u32
}

impl BiomeSurfaceMaterialData {
    pub fn new(top_block: u32, mid_block: u32, sea_floor_block: u32, foundation_block: u32, sea_block: u32, sea_floor_depth: u32) -> Self {
        BiomeSurfaceMaterialData { top_block, mid_block, sea_floor_block, foundation_block, sea_block, sea_floor_depth }
    }

    pub fn read(stream: &mut Stream) -> BiomeSurfaceMaterialData {
        let top_block = stream.get_l_int();
        let mid_block = stream.get_l_int();
        let sea_floor_block = stream.get_l_int();
        let foundation_block = stream.get_l_int();
        let sea_block = stream.get_l_int();
        let sea_floor_depth = stream.get_l_int();

        BiomeSurfaceMaterialData::new(top_block, mid_block, sea_floor_block, foundation_block, sea_block, sea_floor_depth)
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_l_int(self.top_block);
        stream.put_l_int(self.mid_block);
        stream.put_l_int(self.sea_floor_block);
        stream.put_l_int(self.foundation_block);
        stream.put_l_int(self.sea_block);
        stream.put_l_int(self.sea_floor_depth);
    }
}