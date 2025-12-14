use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct DataStoreValueDouble {
    pub value: f64,
}

impl DataStoreValueDouble {
    pub fn new(value: f64) -> DataStoreValueDouble {
        DataStoreValueDouble { value }
    }

    pub fn read(stream: &mut Stream) -> DataStoreValueDouble {
        let value = stream.get_f64_le();

        DataStoreValueDouble { value }
    }

    pub fn write(&mut self, stream: &mut Stream) {
        stream.put_f64_le(self.value);
    }
}
