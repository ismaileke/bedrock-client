use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};
use crate::protocol::bedrock::types::attribute_value_color_array::AttributeValueColorArray;
use crate::protocol::bedrock::types::attribute_value_color_string::AttributeValueColorString;
use crate::protocol::bedrock::types::attribute_value_color_value::AttributeValueColorValue;

#[derive(serde::Serialize, Debug)]
pub struct AttributeValueColor {
    pub value: AttributeValueColorValue,
    pub operation: String
}

impl AttributeValueColor {
    pub const OPERATION_OVERRIDE: &str = "override";
    pub const OPERATION_ALPHA_BLEND: &str = "alpha_blend";
    pub const OPERATION_ADD: &str = "add";
    pub const OPERATION_SUBTRACT: &str = "subtract";
    pub const OPERATION_MULTIPLY: &str = "multiply";

    pub fn new(value: AttributeValueColorValue, operation: String) -> AttributeValueColor {
        AttributeValueColor { value, operation }
    }

    pub fn read(stream: &mut Reader) -> AttributeValueColor {
        let value_type = stream.get_var_u32();
        let value = match value_type {
            AttributeValueColorValue::STRING => AttributeValueColorValue::String(AttributeValueColorString::read(stream)),
            AttributeValueColorValue::ARRAY => AttributeValueColorValue::Array(AttributeValueColorArray::read(stream)),
            _ => panic!("Invalid AttributeValueColorValue type in AttributeValueColor: {}", value_type),
        };
        let operation = PacketSerializer::get_string(stream);

        AttributeValueColor { value, operation }
    }

    pub fn write(&self, stream: &mut Writer) {
        stream.put_var_u32(self.value.id());
        self.value.write(stream);
        PacketSerializer::put_string(stream, &self.operation);
    }
}
