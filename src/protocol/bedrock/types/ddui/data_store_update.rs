use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::ddui::update::data_store_value::DataStoreUpdateValue;
use crate::protocol::bedrock::types::ddui::update::data_store_value_bool::DataStoreUpdateValueBool;
use crate::protocol::bedrock::types::ddui::update::data_store_value_double::DataStoreUpdateValueDouble;
use crate::protocol::bedrock::types::ddui::update::data_store_value_string::DataStoreUpdateValueString;
use crate::protocol::bedrock::types::ddui::update::data_store_value_types::DataStoreUpdateValueTypes;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct DataStoreUpdate {
    pub name: String,
    pub property: String,
    pub path: String,
    pub data: DataStoreUpdateValue,
    pub update_count: u32,
    pub path_update_count: u32
}

impl DataStoreUpdate {
    pub fn new(name: String, property: String, path: String, data: DataStoreUpdateValue, update_count: u32, path_update_count: u32) -> DataStoreUpdate {
        DataStoreUpdate { name, property, path, data, update_count, path_update_count }
    }

    pub fn read(stream: &mut Reader) -> DataStoreUpdate {
        let name = PacketSerializer::get_string(stream);
        let property = PacketSerializer::get_string(stream);
        let path = PacketSerializer::get_string(stream);
        let data_type = stream.get_var_u32();
        let data = match data_type {
            DataStoreUpdateValueTypes::DOUBLE => { DataStoreUpdateValue::Double(DataStoreUpdateValueDouble::read(stream)) }
            DataStoreUpdateValueTypes::STRING => { DataStoreUpdateValue::String(DataStoreUpdateValueString::read(stream)) }
            DataStoreUpdateValueTypes::BOOL => DataStoreUpdateValue::Bool(DataStoreUpdateValueBool::read(stream)),
            _ => panic!("Unknown update data store value type: {}", data_type),
        };
        let update_count = stream.get_u32_le();
        let path_update_count = stream.get_u32_le();

        DataStoreUpdate { name, property, path, data, update_count, path_update_count }
    }

    pub fn write(&mut self, stream: &mut Writer) {
        PacketSerializer::put_string(stream, &self.name);
        PacketSerializer::put_string(stream, &self.property);
        PacketSerializer::put_string(stream, &self.path);
        stream.put_var_u32(self.data.get_type_id());
        self.data.write(stream);
        stream.put_u32_le(self.update_count);
        stream.put_u32_le(self.path_update_count);
    }
}
