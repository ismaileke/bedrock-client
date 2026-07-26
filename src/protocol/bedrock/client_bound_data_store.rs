use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::types::ddui::data_store_change::DataStoreChange;
use crate::protocol::bedrock::types::ddui::data_store_removal::DataStoreRemoval;
use crate::protocol::bedrock::types::ddui::data_store_update::DataStoreUpdate;
use crate::protocol::bedrock::types::ddui::data_store_operation::DataStoreOperation;
use crate::protocol::bedrock::types::ddui::data_store_types::DataStoreOperationTypes;
use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct ClientBoundDataStore {
    pub values: Vec<DataStoreOperation>,
}

impl Packet for ClientBoundDataStore {
    fn id(&self) -> u16 {
        BedrockPacketType::IDClientBoundDataStore.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_var_u32(self.values.len() as u32);
        for value in self.values.iter_mut() {
            stream.put_var_u32(value.get_type_id());
            value.write(&mut stream);
        }

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> ClientBoundDataStore {
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
