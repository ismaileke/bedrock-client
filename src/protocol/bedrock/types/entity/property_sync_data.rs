use std::collections::HashMap;
use binary_utils::binary::Stream;

#[derive(Debug)]
pub struct PropertySyncData {
    int_properties: HashMap<u32, i32>,
    float_properties: HashMap<u32, f32>
}

impl PropertySyncData {
    pub fn new(int_properties: HashMap<u32, i32>, float_properties: HashMap<u32, f32>) -> PropertySyncData {
        PropertySyncData{ int_properties, float_properties }
    }

    pub fn read(stream: &mut Stream) -> PropertySyncData {
        let mut int_properties = HashMap::new();
        let mut float_properties = HashMap::new();

        let int_count = stream.get_unsigned_var_int();
        for _ in 0..int_count {
            let key = stream.get_unsigned_var_int();
            let value = stream.get_var_int();
            int_properties.insert(key, value);
        }
        let float_count = stream.get_unsigned_var_int();
        for _ in 0..float_count {
            let key = stream.get_unsigned_var_int();
            let value = stream.get_l_float();
            float_properties.insert(key, value);
        }

        PropertySyncData{ int_properties, float_properties }
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_unsigned_var_int(self.int_properties.len() as u32);
        for (key, value) in self.int_properties.iter() {
            stream.put_unsigned_var_int(*key);
            stream.put_var_int(*value);
        }
        stream.put_unsigned_var_int(self.float_properties.len() as u32);
        for (key, value) in self.float_properties.iter() {
            stream.put_unsigned_var_int(*key);
            stream.put_l_float(*value);
        }
    }
}