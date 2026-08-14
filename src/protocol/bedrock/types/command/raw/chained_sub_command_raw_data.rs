use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::command::raw::chained_sub_command_value_raw_data::ChainedSubCommandValueRawData;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct ChainedSubCommandRawData {
    pub name: String,
    pub value_data: Vec<ChainedSubCommandValueRawData>
}

impl ChainedSubCommandRawData {
    pub fn new(name: String, value_data: Vec<ChainedSubCommandValueRawData>) -> ChainedSubCommandRawData {
        ChainedSubCommandRawData { name, value_data }
    }

    pub fn read(stream: &mut Reader) -> ChainedSubCommandRawData {
        let name = PacketSerializer::get_string(stream);
        let size = stream.get_var_u32();
        let mut value_data = Vec::with_capacity(size as usize);
        for _ in 0..size {
            value_data.push(ChainedSubCommandValueRawData::read(stream));
        }

        ChainedSubCommandRawData { name, value_data }
    }

    pub fn write(&self, stream: &mut Writer) {
        PacketSerializer::put_string(stream, &self.name);
        stream.put_var_u32(self.value_data.len() as u32);
        for value_datum in &self.value_data {
            value_datum.write(stream);
        }
    }
}
