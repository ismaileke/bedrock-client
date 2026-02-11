use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::data_store_value::DataStoreValue;
use crate::protocol::bedrock::types::data_store_value_bool::DataStoreValueBool;
use crate::protocol::bedrock::types::data_store_value_double::DataStoreValueDouble;
use crate::protocol::bedrock::types::data_store_value_string::DataStoreValueString;
use crate::protocol::bedrock::types::data_store_value_types::DataStoreValueTypes;
use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct DataStoreUpdate {
    pub name: String,
    pub property: String,
    pub path: String,
    pub data: DataStoreValue,
    pub update_count: u32,
    pub path_update_count: u32
}

impl DataStoreUpdate {
    pub fn new(name: String, property: String, path: String, data: DataStoreValue, update_count: u32, path_update_count: u32) -> DataStoreUpdate {
        DataStoreUpdate { name, property, path, data, update_count, path_update_count }
    }

    pub fn read(stream: &mut Stream) -> DataStoreUpdate {
        let name = PacketSerializer::get_string(stream);
        let property = PacketSerializer::get_string(stream);
        let path = PacketSerializer::get_string(stream);

        let data_type = stream.get_var_u32();
        let data = match data_type {
            DataStoreValueTypes::DOUBLE => {
                DataStoreValue::Double(DataStoreValueDouble::read(stream))
            }
            DataStoreValueTypes::STRING => {
                DataStoreValue::String(DataStoreValueString::read(stream))
            }
            DataStoreValueTypes::BOOL => DataStoreValue::Bool(DataStoreValueBool::read(stream)),
            _ => panic!("Unknown data store value type: {}", data_type),
        };
        let update_count = stream.get_u32_le();
        let path_update_count = stream.get_u32_le();

        DataStoreUpdate { name, property, path, data, update_count, path_update_count }
    }

    pub fn write(&mut self, stream: &mut Stream) {
        PacketSerializer::put_string(stream, self.name.clone());
        PacketSerializer::put_string(stream, self.property.clone());
        PacketSerializer::put_string(stream, self.path.clone());
        stream.put_var_u32(self.data.get_type_id());
        self.data.write(stream);
        stream.put_u32_le(self.update_count);
        stream.put_u32_le(self.path_update_count);
    }
}
