use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct MemoryCategoryCounter {
    pub category: u8,
    pub bytes: u64
}

impl MemoryCategoryCounter {
    pub fn new(category: u8, bytes: u64) -> MemoryCategoryCounter {
        MemoryCategoryCounter { category, bytes }
    }

    pub fn read(stream: &mut Stream) -> MemoryCategoryCounter {
        let category = stream.get_byte();
        let bytes = stream.get_u64_le();

        MemoryCategoryCounter { category, bytes }
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_byte(self.category);
        stream.put_u64_le(self.bytes);
    }
}