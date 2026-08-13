use crate::protocol::bedrock::types::cereal::dynamic_value_bool::DynamicValueBool;
use crate::protocol::bedrock::types::cereal::dynamic_value_double::DynamicValueDouble;
use crate::protocol::bedrock::types::cereal::dynamic_value_list::DynamicValueList;
use crate::protocol::bedrock::types::cereal::dynamic_value_long::DynamicValueLong;
use crate::protocol::bedrock::types::cereal::dynamic_value_map::DynamicValueMap;
use crate::protocol::bedrock::types::cereal::dynamic_value_string::DynamicValueString;
use binary_utils::binary::{Reader, Writer};
use std::fmt::Debug;

#[derive(serde::Serialize, Debug)]
pub enum DynamicValue {
    Null(),
    Bool(DynamicValueBool),
    Long(DynamicValueLong),
    Double(DynamicValueDouble),
    String(DynamicValueString),
    List(DynamicValueList),
    Map(DynamicValueMap),
}

impl DynamicValue {
    pub const NULL: u32 = 0;
    pub const BOOL: u32 = 1;
    pub const LONG: u32 = 2;
    pub const DOUBLE: u32 = 3;
    pub const STRING: u32 = 4;
    pub const LIST: u32 = 5;
    pub const MAP: u32 = 6;

    pub fn id(&self) -> u32 {
        match self {
            DynamicValue::Null() => Self::NULL,
            DynamicValue::Bool(_) => Self::BOOL,
            DynamicValue::Long(_) => Self::LONG,
            DynamicValue::Double(_) => Self::DOUBLE,
            DynamicValue::String(_) => Self::STRING,
            DynamicValue::List(_) => Self::LIST,
            DynamicValue::Map(_) => Self::MAP,
        }
    }

    pub fn read(stream: &mut Reader, dynamic_value_type: u32) -> DynamicValue {
        match dynamic_value_type {
            DynamicValue::NULL => DynamicValue::Null(),
            DynamicValue::BOOL => DynamicValue::Bool(DynamicValueBool::read(stream)),
            DynamicValue::LONG => DynamicValue::Long(DynamicValueLong::read(stream)),
            DynamicValue::DOUBLE => DynamicValue::Double(DynamicValueDouble::read(stream)),
            DynamicValue::STRING => DynamicValue::String(DynamicValueString::read(stream)),
            DynamicValue::LIST => DynamicValue::List(DynamicValueList::read(stream)),
            DynamicValue::MAP => DynamicValue::Map(DynamicValueMap::read(stream)),
            _ => panic!("Unknown dynamic value type: {}", dynamic_value_type),
        }
    }

    pub fn write(&self, stream: &mut Writer) {
        match self {
            DynamicValue::Null() => (),
            DynamicValue::Bool(r) => r.write(stream),
            DynamicValue::Long(r) => r.write(stream),
            DynamicValue::Double(r) => r.write(stream),
            DynamicValue::String(r) => r.write(stream),
            DynamicValue::List(r) => r.write(stream),
            DynamicValue::Map(r) => r.write(stream),
        }
    }
}
