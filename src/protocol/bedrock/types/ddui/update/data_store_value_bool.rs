use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct DataStoreUpdateValueBool {
    pub value: bool
}

impl DataStoreUpdateValueBool {
    pub fn new(value: bool) -> DataStoreUpdateValueBool {
        DataStoreUpdateValueBool { value }
    }

    pub fn read(stream: &mut Reader) -> DataStoreUpdateValueBool {
        let value = stream.get_bool();

        DataStoreUpdateValueBool { value }
    }

    pub fn write(&mut self, stream: &mut Writer) {
        stream.put_bool(self.value);
    }
}
