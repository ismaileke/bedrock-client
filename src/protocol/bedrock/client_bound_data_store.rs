use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::types::ddui::data_store_change::DataStoreChange;
use crate::protocol::bedrock::types::ddui::data_store_removal::DataStoreRemoval;
use crate::protocol::bedrock::types::ddui::data_store_update::DataStoreUpdate;
use crate::protocol::bedrock::types::ddui::data_store_operation::DataStoreOperation;
use crate::protocol::bedrock::types::ddui::data_store_types::DataStoreOperationTypes;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct ClientBoundDataStore {
    pub values: Vec<DataStoreOperation>,
}

impl Packet for ClientBoundDataStore {
    fn id(&self) -> u16 {
        BedrockPacketType::IDClientBoundDataStore.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_var_u32(self.values.len() as u32);
        for value in self.values.iter_mut() {
            stream.put_var_u32(value.get_type_id());
            value.write(stream);
        }
    }

    fn decode(stream: &mut Reader) -> ClientBoundDataStore {
        let mut values = Vec::new();
        let length = stream.get_var_u32();
        for _ in 0..length {
            let value_type = stream.get_var_u32();
            values.push(
                match value_type {
                    DataStoreOperationTypes::UPDATE => DataStoreOperation::Update(DataStoreUpdate::read(stream)),
                    DataStoreOperationTypes::CHANGE => DataStoreOperation::Change(DataStoreChange::read(stream)),
                    DataStoreOperationTypes::REMOVAL => DataStoreOperation::Removal(DataStoreRemoval::read(stream)),
                    _ => panic!("Unknown data store type {}", value_type),
                }
            );
        }

        ClientBoundDataStore { values }
    }
}
