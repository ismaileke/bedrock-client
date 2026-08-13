use crate::protocol::bedrock::types::ddui::data_store_change::DataStoreChange;
use crate::protocol::bedrock::types::ddui::data_store_removal::DataStoreRemoval;
use crate::protocol::bedrock::types::ddui::data_store_types::DataStoreOperationTypes;
use crate::protocol::bedrock::types::ddui::data_store_update::DataStoreUpdate;
use binary_utils::binary::Writer;
use std::fmt::Debug;

#[derive(serde::Serialize, Debug)]
pub enum DataStoreOperation {
    Update(DataStoreUpdate),
    Change(DataStoreChange),
    Removal(DataStoreRemoval),
}

impl DataStoreOperation {
    pub fn get_type_id(&self) -> u32 {
        match self {
            DataStoreOperation::Update(_) => DataStoreOperationTypes::UPDATE,
            DataStoreOperation::Change(_) => DataStoreOperationTypes::CHANGE,
            DataStoreOperation::Removal(_) => DataStoreOperationTypes::REMOVAL,
        }
    }

    pub fn write(&mut self, stream: &mut Writer) {
        match self {
            DataStoreOperation::Update(r) => r.write(stream),
            DataStoreOperation::Change(r) => r.write(stream),
            DataStoreOperation::Removal(r) => r.write(stream),
        }
    }
}
