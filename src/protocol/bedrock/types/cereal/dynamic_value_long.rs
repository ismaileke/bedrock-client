use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct DynamicValueLong {
    pub value: i64
}

impl DynamicValueLong {

    pub fn new(value: i64) -> DynamicValueLong {
        DynamicValueLong { value }
    }

    pub fn read(stream: &mut Reader) -> DynamicValueLong {
        let value = stream.get_i64_le();

        DynamicValueLong { value }
    }

    pub fn write(&self, stream: &mut Writer) {
        stream.put_i64_le(self.value);
    }
}
