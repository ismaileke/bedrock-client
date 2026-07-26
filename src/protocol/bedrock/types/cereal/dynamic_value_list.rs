use crate::protocol::bedrock::types::cereal::dynamic_value::DynamicValue;
use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct DynamicValueList {
    pub values: Vec<DynamicValue>
}

impl DynamicValueList {

    pub fn new(values: Vec<DynamicValue>) -> DynamicValueList {
        DynamicValueList { values }
    }

    pub fn read(stream: &mut Stream) -> DynamicValueList {
        let size = stream.get_var_u32();
        let mut values = Vec::with_capacity(size as usize);
        for _ in 0..size {
            let dynamic_value_type = stream.get_u32_le();
            values.push(DynamicValue::read(stream, dynamic_value_type));
        }

        DynamicValueList { values }
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_var_u32(self.values.len() as u32);
        for value in &self.values {
            stream.put_u32_le(value.id());
            value.write(stream);
        }
    }
}
