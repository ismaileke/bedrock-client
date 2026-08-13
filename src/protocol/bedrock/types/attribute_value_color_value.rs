use crate::protocol::bedrock::types::attribute_value_color_array::AttributeValueColorArray;
use crate::protocol::bedrock::types::attribute_value_color_string::AttributeValueColorString;
use binary_utils::binary::Writer;
use std::fmt::Debug;

#[derive(serde::Serialize, Debug)]
pub enum AttributeValueColorValue {
    String(AttributeValueColorString),
    Array(AttributeValueColorArray)
}

impl AttributeValueColorValue {
    pub const STRING: u32 = 0;
    pub const ARRAY: u32 = 1;

    pub fn id(&self) -> u32 {
        match self {
            AttributeValueColorValue::String(_) => Self::STRING,
            AttributeValueColorValue::Array(_) => Self::ARRAY,
        }
    }

    pub fn write(&self, stream: &mut Writer) {
        match self {
            AttributeValueColorValue::String(r) => r.write(stream),
            AttributeValueColorValue::Array(r) => r.write(stream),
        }
    }
}
