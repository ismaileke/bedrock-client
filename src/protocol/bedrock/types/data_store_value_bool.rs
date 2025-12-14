use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct DataStoreValueBool {
    pub value: bool,
}

impl DataStoreValueBool {
    pub fn new(value: bool) -> DataStoreValueBool {
        DataStoreValueBool { value }
    }

    pub fn read(stream: &mut Stream) -> DataStoreValueBool {
        let value = stream.get_bool();

        DataStoreValueBool { value }
    }

    pub fn write(&mut self, stream: &mut Stream) {
        stream.put_bool(self.value);
    }
}
