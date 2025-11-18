use binary_utils::binary::Stream;
use mojang_nbt::base_nbt_serializer::BaseNBTSerializer;

pub struct NetworkNBTSerializer {
    binary_stream: Stream
}

impl NetworkNBTSerializer {
    pub fn new() -> NetworkNBTSerializer {
        NetworkNBTSerializer{ binary_stream: Stream::new(vec![], 0) }
    }
}

impl BaseNBTSerializer for NetworkNBTSerializer {

    fn get_stream(&mut self) -> &mut Stream {
        &mut self.binary_stream
    }

    fn read_short(&mut self) -> i16 {
        self.binary_stream.get_i16_le()
    }

    fn read_signed_short(&mut self) -> i16 {
        self.binary_stream.get_i16_le() //????????
    }

    fn read_int(&mut self) -> i32 {
        self.binary_stream.get_var_i32()
    }

    fn read_long(&mut self) -> i64 {
        self.binary_stream.get_var_i64()
    }

    fn read_float(&mut self) -> f32 {
        self.binary_stream.get_f32_le()
    }

    fn read_double(&mut self) -> f64 {
        self.binary_stream.get_f64_le()
    }

    fn read_int_array(&mut self) -> Vec<i32> {
        let len = self.read_int();

        let mut int_array = Vec::new();

        for _ in 0..len {
            int_array.push(self.read_int());
        }

        int_array
    }

    fn write_short(&mut self, data: i16) {
        self.binary_stream.put_i16_le(data)
    }

    fn write_int(&mut self, data: i32) {
        self.binary_stream.put_var_i32(data)
    }

    fn write_long(&mut self, data: i64) {
        self.binary_stream.put_var_i64(data)
    }

    fn write_float(&mut self, value: f32) {
        self.binary_stream.put_f32_le(value)
    }

    fn write_double(&mut self, data: f64) {
        self.binary_stream.put_f64_le(data)
    }

    fn write_int_array(&mut self, data: Vec<i32>) {
        self.write_int(data.len() as i32);

        for datum in &data {
            self.write_int(*datum);
        }
    }

    fn read_string(&mut self) -> String {
        let len = self.binary_stream.get_var_u32();
        String::from_utf8(self.binary_stream.get(len).to_vec()).expect("read_string() fn error, network_nbt_serializer (Vec to String error)")
    }

    fn write_string(&mut self, value: String) {
        self.binary_stream.put_var_u32(value.len() as u32);
        self.binary_stream.put(value.into_bytes());
    }
}