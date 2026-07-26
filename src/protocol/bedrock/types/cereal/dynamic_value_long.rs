use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct DynamicValueLong {
    pub value: i64
}

impl DynamicValueLong {

    pub fn new(value: i64) -> DynamicValueLong {
        DynamicValueLong { value }
    }

    pub fn read(stream: &mut Stream) -> DynamicValueLong {
        let value = stream.get_i64_le();

        DynamicValueLong { value }
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_i64_le(self.value);
    }
}
