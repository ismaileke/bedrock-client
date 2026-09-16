use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct NoiseAlignment {
    pub alignment_type: u8,
    pub value: u32
}

impl NoiseAlignment {
    pub const MIN_LOCAL_TRANSITION_END: u8 = 0;

    pub fn new(alignment_type: u8, value: u32) -> NoiseAlignment {
        NoiseAlignment { alignment_type, value }
    }

    pub fn read(stream: &mut Reader) -> NoiseAlignment {
        let alignment_type = stream.get_u8();
        let value = stream.get_var_u32();
        NoiseAlignment { alignment_type, value }
    }

    pub fn write(&self, stream: &mut Writer) {
        stream.put_u8(self.alignment_type);
        stream.put_var_u32(self.value);
    }
}
