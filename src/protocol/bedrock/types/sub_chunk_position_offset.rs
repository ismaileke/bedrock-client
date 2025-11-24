use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct SubChunkPositionOffset {
    pub x_offset: u8,
    pub y_offset: u8,
    pub z_offset: u8
}

pub fn new(x_offset: u8, y_offset: u8, z_offset: u8) -> SubChunkPositionOffset {
    SubChunkPositionOffset{ x_offset: clamp_offset(x_offset), y_offset: clamp_offset(y_offset), z_offset: clamp_offset(z_offset) }
}

fn clamp_offset(offset: u8) -> u8 {
    if offset < u8::MIN || offset > u8::MAX { // OR i8?
        panic!("Offsets must be within the range of a byte ({}...{})", u8::MIN, u8::MAX);
    }
    offset
}

impl SubChunkPositionOffset {
    pub fn read(stream: &mut Stream) -> SubChunkPositionOffset {

        let x_offset = stream.get_byte();
        let y_offset = stream.get_byte();
        let z_offset = stream.get_byte();

        SubChunkPositionOffset{ x_offset, y_offset, z_offset }
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_byte(self.x_offset);
        stream.put_byte(self.y_offset);
        stream.put_byte(self.z_offset);
    }
}