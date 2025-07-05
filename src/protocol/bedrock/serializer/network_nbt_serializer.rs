use binary_utils::binary::Stream;
use mojang_nbt::base_nbt_serializer::BaseNBTSerializer;

pub struct NetworkNBTSerializer {
    binary_stream: Stream,
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

    fn read_short(&mut self) -> u16 {
        self.binary_stream.get_l_short()
    }

    fn read_signed_short(&mut self) -> i16 {
        self.binary_stream.get_signed_l_short()
    }
    fn read_int(&mut self) -> u32 {
        self.binary_stream.get_var_int() as u32 // EDIT LATER
    }

    fn read_long(&mut self) -> i64 {
        self.binary_stream.get_var_long()
    }

    fn read_float(&mut self) -> f32 {
        self.binary_stream.get_l_float()
    }

    fn read_double(&mut self) -> f64 {
        self.binary_stream.get_l_double()
    }

    fn read_int_array(&mut self) -> Vec<u32> {
        let len = self.read_int();

        let mut int_array = Vec::new();

        for _ in 0..len {
            int_array.push(self.read_int());
        }

        int_array
    }

    fn write_short(&mut self, data: u16) {
        self.binary_stream.put_l_short(data)
    }

    fn write_int(&mut self, data: u32) {
        self.binary_stream.put_var_int(data as i32) // EDIT LATER
    }

    fn write_long(&mut self, data: i64) {
        self.binary_stream.put_var_long(data)
    }

    fn write_float(&mut self, value: f32) {
        self.binary_stream.put_l_float(value)
    }

    fn write_double(&mut self, data: f64) {
        self.binary_stream.put_l_double(data)
    }

    fn write_int_array(&mut self, data: Vec<u32>) {
        self.write_int(data.len() as u32);

        for datum in &data {
            self.write_int(*datum);
        }
    }

    fn read_string(&mut self) -> String {
        let len = self.binary_stream.get_unsigned_var_int();
        String::from_utf8(self.binary_stream.get(len).expect("read_string() fn error, network_nbt_serializer (BinaryStream get() error)").to_vec()).expect("read_string() fn error, network_nbt_serializer (Vec to String error)")
    }

    fn write_string(&mut self, value: String) {
        self.binary_stream.put_unsigned_var_int(value.len() as u32);
        self.binary_stream.put(value.into_bytes());
    }
}