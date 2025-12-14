use crate::protocol::bedrock::types::data_store_change::DataStoreChange;
use crate::protocol::bedrock::types::data_store_removal::DataStoreRemoval;
use crate::protocol::bedrock::types::data_store_types::DataStoreTypes;
use crate::protocol::bedrock::types::data_store_update::DataStoreUpdate;
use binary_utils::binary::Stream;
use std::fmt::Debug;

#[derive(serde::Serialize, Debug)]
pub enum DataStore {
    Update(DataStoreUpdate),
    Change(DataStoreChange),
    Removal(DataStoreRemoval),
}

impl DataStore {
    pub fn get_type_id(&self) -> u32 {
        match self {
            DataStore::Update(_) => DataStoreTypes::UPDATE,
            DataStore::Change(_) => DataStoreTypes::CHANGE,
            DataStore::Removal(_) => DataStoreTypes::REMOVAL,
        }
    }

    pub fn write(&mut self, stream: &mut Stream) {
        match self {
            DataStore::Update(r) => r.write(stream),
            DataStore::Change(r) => r.write(stream),
            DataStore::Removal(r) => r.write(stream),
        }
    }
}
