use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct DynamicValueDouble {
    pub value: f64
}

impl DynamicValueDouble {

    pub fn new(value: f64) -> DynamicValueDouble {
        DynamicValueDouble { value }
    }

    pub fn read(stream: &mut Reader) -> DynamicValueDouble {
        let value = stream.get_f64_le();

        DynamicValueDouble { value }
    }

    pub fn write(&self, stream: &mut Writer) {
        stream.put_f64_le(self.value);
    }
}
