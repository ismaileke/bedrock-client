use crate::protocol::bedrock::types::data_store_value_bool::DataStoreValueBool;
use crate::protocol::bedrock::types::data_store_value_double::DataStoreValueDouble;
use crate::protocol::bedrock::types::data_store_value_string::DataStoreValueString;
use crate::protocol::bedrock::types::data_store_value_types::DataStoreValueTypes;
use binary_utils::binary::Stream;
use std::fmt::Debug;

#[derive(serde::Serialize, Debug)]
pub enum DataStoreValue {
    Double(DataStoreValueDouble),
    Bool(DataStoreValueBool),
    String(DataStoreValueString),
}

impl DataStoreValue {
    pub fn get_type_id(&self) -> u32 {
        match self {
            DataStoreValue::Double(_) => DataStoreValueTypes::DOUBLE,
            DataStoreValue::Bool(_) => DataStoreValueTypes::BOOL,
            DataStoreValue::String(_) => DataStoreValueTypes::STRING,
        }
    }

    pub fn write(&mut self, stream: &mut Stream) {
        match self {
            DataStoreValue::Double(r) => r.write(stream),
            DataStoreValue::Bool(r) => r.write(stream),
            DataStoreValue::String(r) => r.write(stream),
        }
    }
}
