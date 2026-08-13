use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct SubChunkPositionOffset {
    pub x_offset: u8,
    pub y_offset: u8,
    pub z_offset: u8,
}

pub fn new(x_offset: u8, y_offset: u8, z_offset: u8) -> SubChunkPositionOffset {
    SubChunkPositionOffset {
        x_offset: clamp_offset(x_offset),
        y_offset: clamp_offset(y_offset),
        z_offset: clamp_offset(z_offset),
    }
}

fn clamp_offset(offset: u8) -> u8 {
    if offset < u8::MIN || offset > u8::MAX {
        // OR i8?
        panic!(
            "Offsets must be within the range of a byte ({}...{})",
            u8::MIN,
            u8::MAX
        );
    }
    offset
}

impl SubChunkPositionOffset {
    pub fn read(stream: &mut Reader) -> SubChunkPositionOffset {
        let x_offset = stream.get_u8();
        let y_offset = stream.get_u8();
        let z_offset = stream.get_u8();

        SubChunkPositionOffset {
            x_offset,
            y_offset,
            z_offset,
        }
    }

    pub fn write(&self, stream: &mut Writer) {
        stream.put_u8(self.x_offset);
        stream.put_u8(self.y_offset);
        stream.put_u8(self.z_offset);
    }
}
