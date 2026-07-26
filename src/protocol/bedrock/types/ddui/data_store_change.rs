use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::cereal::dynamic_value::DynamicValue;
use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct DataStoreChange {
    pub name: String,
    pub property: String,
    pub update_count: u32,
    pub data: DynamicValue,
}

impl DataStoreChange {
    pub fn new(name: String, property: String, update_count: u32, data: DynamicValue) -> DataStoreChange {
        DataStoreChange { name, property, update_count, data }
    }

    pub fn read(stream: &mut Stream) -> DataStoreChange {
        let name = PacketSerializer::get_string(stream);
        let property = PacketSerializer::get_string(stream);
        let update_count = stream.get_var_u32();
        let dynamic_value_type = stream.get_u32_le();
        let data = DynamicValue::read(stream, dynamic_value_type);

        DataStoreChange { name, property, update_count, data }
    }

    pub fn write(&mut self, stream: &mut Stream) {
        PacketSerializer::put_string(stream, self.name.clone());
        PacketSerializer::put_string(stream, self.property.clone());
        stream.put_var_u32(self.update_count);
        stream.put_u32_le(self.data.id());
        self.data.write(stream);
    }
}
