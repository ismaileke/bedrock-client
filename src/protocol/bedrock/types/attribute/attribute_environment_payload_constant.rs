use binary_utils::binary::{Reader, Writer};
use crate::protocol::bedrock::types::attribute::attribute_value::AttributeValue;

#[derive(serde::Serialize, Debug)]
pub struct AttributeEnvironmentPayloadConstant {
    pub attribute: AttributeValue
}

impl AttributeEnvironmentPayloadConstant {
    pub fn read(stream: &mut Reader) -> AttributeEnvironmentPayloadConstant {
        let attribute = AttributeValue::read(stream);
        AttributeEnvironmentPayloadConstant { attribute }
    }

    pub fn write(&self, stream: &mut Writer) {
        stream.put_var_u32(self.attribute.id());
        self.attribute.write(stream);
    }
}
