use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct SubChunkPosition {
    x: i32,
    y: i32,
    z: i32
}

impl SubChunkPosition {
    pub fn new(x: i32, y: i32, z: i32) -> SubChunkPosition {
        SubChunkPosition { x, y, z }
    }

    pub fn read_fixed_ints(stream: &mut Stream) -> SubChunkPosition {
        let x = stream.get_i32_le();
        let y = stream.get_i32_le();
        let z = stream.get_i32_le();
        SubChunkPosition { x, y, z }
    }

    pub fn write_fixed_ints(&self, stream: &mut Stream) {
        stream.put_i32_le(self.x);
        stream.put_i32_le(self.y);
        stream.put_i32_le(self.z);
    }

    pub fn read_var_ints(stream: &mut Stream) -> SubChunkPosition {
        let x = stream.get_var_i32();
        let y = stream.get_var_i32();
        let z = stream.get_var_i32();
        SubChunkPosition { x, y, z }
    }

    pub fn write_var_ints(&self, stream: &mut Stream) {
        stream.put_var_i32(self.x);
        stream.put_var_i32(self.y);
        stream.put_var_i32(self.z);
    }
}
