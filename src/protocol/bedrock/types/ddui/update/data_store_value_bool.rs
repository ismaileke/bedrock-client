use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct DataStoreUpdateValueBool {
    pub value: bool
}

impl DataStoreUpdateValueBool {
    pub fn new(value: bool) -> DataStoreUpdateValueBool {
        DataStoreUpdateValueBool { value }
    }

    pub fn read(stream: &mut Stream) -> DataStoreUpdateValueBool {
        let value = stream.get_bool();

        DataStoreUpdateValueBool { value }
    }

    pub fn write(&mut self, stream: &mut Stream) {
        stream.put_bool(self.value);
    }
}
