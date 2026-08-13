use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct DataStoreUpdateValueDouble {
    pub value: f64
}

impl DataStoreUpdateValueDouble {
    pub fn new(value: f64) -> DataStoreUpdateValueDouble {
        DataStoreUpdateValueDouble { value }
    }

    pub fn read(stream: &mut Reader) -> DataStoreUpdateValueDouble {
        let value = stream.get_f64_le();

        DataStoreUpdateValueDouble { value }
    }

    pub fn write(&mut self, stream: &mut Writer) {
        stream.put_f64_le(self.value);
    }
}
