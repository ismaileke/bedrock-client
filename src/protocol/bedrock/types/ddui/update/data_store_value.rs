use crate::protocol::bedrock::types::ddui::update::data_store_value_bool::DataStoreUpdateValueBool;
use crate::protocol::bedrock::types::ddui::update::data_store_value_double::DataStoreUpdateValueDouble;
use crate::protocol::bedrock::types::ddui::update::data_store_value_string::DataStoreUpdateValueString;
use crate::protocol::bedrock::types::ddui::update::data_store_value_types::DataStoreUpdateValueTypes;
use binary_utils::binary::Writer;
use std::fmt::Debug;

#[derive(serde::Serialize, Debug)]
pub enum DataStoreUpdateValue {
    Double(DataStoreUpdateValueDouble),
    Bool(DataStoreUpdateValueBool),
    String(DataStoreUpdateValueString),
}

impl DataStoreUpdateValue {
    pub fn get_type_id(&self) -> u32 {
        match self {
            DataStoreUpdateValue::Double(_) => DataStoreUpdateValueTypes::DOUBLE,
            DataStoreUpdateValue::Bool(_) => DataStoreUpdateValueTypes::BOOL,
            DataStoreUpdateValue::String(_) => DataStoreUpdateValueTypes::STRING,
        }
    }

    pub fn write(&mut self, stream: &mut Writer) {
        match self {
            DataStoreUpdateValue::Double(r) => r.write(stream),
            DataStoreUpdateValue::Bool(r) => r.write(stream),
            DataStoreUpdateValue::String(r) => r.write(stream),
        }
    }
}
