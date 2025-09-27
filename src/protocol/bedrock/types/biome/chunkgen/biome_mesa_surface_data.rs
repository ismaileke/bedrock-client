use binary_utils::binary::Stream;

#[derive(Debug)]
pub struct BiomeMesaSurfaceData {
    clay_material: u32,
    hard_clay_material: u32,
    bryce_pillars: bool,
    forest: bool
}

impl BiomeMesaSurfaceData {
    pub fn new(clay_material: u32, hard_clay_material: u32, bryce_pillars: bool, forest: bool) -> Self {
        BiomeMesaSurfaceData{ clay_material, hard_clay_material, bryce_pillars, forest }
    }

    pub fn read(stream: &mut Stream) -> BiomeMesaSurfaceData {
        let clay_material = stream.get_l_int();
        let hard_clay_material = stream.get_l_int();
        let bryce_pillars = stream.get_bool();
        let forest = stream.get_bool();

        BiomeMesaSurfaceData::new(clay_material, hard_clay_material, bryce_pillars, forest)
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_l_int(self.clay_material);
        stream.put_l_int(self.hard_clay_material);
        stream.put_bool(self.bryce_pillars);
        stream.put_bool(self.forest);
    }
}