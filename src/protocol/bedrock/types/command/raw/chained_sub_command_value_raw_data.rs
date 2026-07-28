use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct ChainedSubCommandValueRawData {
    pub name_index: u32,
    pub data_type: u32
}

impl ChainedSubCommandValueRawData {
    pub fn new(name_index: u32, data_type: u32) -> ChainedSubCommandValueRawData {
        ChainedSubCommandValueRawData { name_index, data_type }
    }

    pub fn read(stream: &mut Stream) -> ChainedSubCommandValueRawData {
        let name_index = stream.get_var_u32();
        let data_type = stream.get_var_u32();

        ChainedSubCommandValueRawData { name_index, data_type }
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_var_u32(self.name_index);
        stream.put_var_u32(self.data_type);
    }
}
