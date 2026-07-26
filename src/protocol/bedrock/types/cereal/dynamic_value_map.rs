use crate::protocol::bedrock::types::cereal::dynamic_value::DynamicValue;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::Stream;
use std::collections::HashMap;

#[derive(serde::Serialize, Debug)]
pub struct DynamicValueMap {
    pub values: HashMap<String, DynamicValue>
}

impl DynamicValueMap {

    pub fn new(values: HashMap<String, DynamicValue>) -> DynamicValueMap {
        DynamicValueMap { values }
    }

    pub fn read(stream: &mut Stream) -> DynamicValueMap {
        let count = stream.get_var_u32();
        let mut values = HashMap::with_capacity(count as usize);
        for _ in 0..count {
            let key = PacketSerializer::get_string(stream);
            let dynamic_value_type = stream.get_u32_le();
            values.insert(key, DynamicValue::read(stream, dynamic_value_type));
        }

        DynamicValueMap { values }
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_var_u32(self.values.len() as u32);
        for (key, value) in &self.values {
            PacketSerializer::put_string(stream, key.to_string());
            stream.put_u32_le(value.id());
            value.write(stream);
        }
    }
}
