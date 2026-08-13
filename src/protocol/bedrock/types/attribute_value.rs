use crate::protocol::bedrock::types::attribute_value_bool::AttributeValueBool;
use crate::protocol::bedrock::types::attribute_value_color::AttributeValueColor;
use crate::protocol::bedrock::types::attribute_value_float::AttributeValueFloat;
use binary_utils::binary::{Reader, Writer};
use std::fmt::Debug;

#[derive(serde::Serialize, Debug)]
pub enum AttributeValue {
    Bool(AttributeValueBool),
    Float(AttributeValueFloat),
    Color(AttributeValueColor),
}

impl AttributeValue {
    pub const BOOL: u32 = 0;
    pub const FLOAT: u32 = 1;
    pub const COLOR: u32 = 2;

    pub fn id(&self) -> u32 {
        match self {
            AttributeValue::Bool(_) => Self::BOOL,
            AttributeValue::Float(_) => Self::FLOAT,
            AttributeValue::Color(_) => Self::COLOR,
        }
    }

    pub fn read(stream: &mut Reader) -> AttributeValue {
        let attribute_value_type = stream.get_var_u32();
        match attribute_value_type {
            AttributeValue::BOOL => AttributeValue::Bool(AttributeValueBool::read(stream)),
            AttributeValue::FLOAT => AttributeValue::Float(AttributeValueFloat::read(stream)),
            AttributeValue::COLOR => AttributeValue::Color(AttributeValueColor::read(stream)),
            _ => panic!("Attribute value type not handled: {}", attribute_value_type),
        }
    }

    pub fn write(&self, stream: &mut Writer) {
        match self {
            AttributeValue::Bool(r) => r.write(stream),
            AttributeValue::Float(r) => r.write(stream),
            AttributeValue::Color(r) => r.write(stream),
        }
    }
}
