use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::data_store_value::DataStoreValue;
use crate::protocol::bedrock::types::data_store_value_bool::DataStoreValueBool;
use crate::protocol::bedrock::types::data_store_value_double::DataStoreValueDouble;
use crate::protocol::bedrock::types::data_store_value_string::DataStoreValueString;
use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct DataStoreChange {
    pub name: String,
    pub property: String,
    pub update_count: u32,
    pub data: DataStoreValue,
}

impl DataStoreChange {
    pub fn new(
        name: String,
        property: String,
        update_count: u32,
        data: DataStoreValue,
    ) -> DataStoreChange {
        DataStoreChange {
            name,
            property,
            update_count,
            data,
        }
    }

    pub fn read(stream: &mut Stream) -> DataStoreChange {
        let name = PacketSerializer::get_string(stream);
        let property = PacketSerializer::get_string(stream);
        let update_count = stream.get_var_u32();

        let data = if stream.get_remaining().len() == 1 {
            DataStoreValue::Bool(DataStoreValueBool::read(stream))
        } else {
            let offset = stream.get_offset();
            let length = stream.get_var_u32();
            if length as usize == stream.get_remaining().len() {
                DataStoreValue::String(DataStoreValueString::read(stream))
            } else {
                stream.set_offset(offset);
                DataStoreValue::Double(DataStoreValueDouble::read(stream))
            }
        };

        DataStoreChange {
            name,
            property,
            update_count,
            data,
        }
    }

    pub fn write(&mut self, stream: &mut Stream) {
        PacketSerializer::put_string(stream, self.name.clone());
        PacketSerializer::put_string(stream, self.property.clone());
        stream.put_var_u32(self.update_count);
        stream.put_var_u32(self.data.get_type_id());
        self.data.write(stream);
    }
}
