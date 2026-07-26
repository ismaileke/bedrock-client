use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct DynamicValueBool {
    pub value: bool
}

impl DynamicValueBool {

    pub fn new(value: bool) -> DynamicValueBool {
        DynamicValueBool { value }
    }

    pub fn read(stream: &mut Stream) -> DynamicValueBool {
        let value = stream.get_bool();

        DynamicValueBool { value }
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_bool(self.value);
    }
}
