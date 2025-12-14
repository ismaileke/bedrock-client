use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct DimensionDataEntry {
    max_height: i32,
    min_height: i32,
    generator: i32,
}

impl DimensionDataEntry {
    pub fn new(max_height: i32, min_height: i32, generator: i32) -> DimensionDataEntry {
        DimensionDataEntry {
            max_height,
            min_height,
            generator,
        }
    }

    pub fn read(stream: &mut Stream) -> DimensionDataEntry {
        let max_height = stream.get_var_i32();
        let min_height = stream.get_var_i32();
        let generator = stream.get_var_i32();

        DimensionDataEntry {
            max_height,
            min_height,
            generator,
        }
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_var_i32(self.max_height);
        stream.put_var_i32(self.min_height);
        stream.put_var_i32(self.generator);
    }
}
