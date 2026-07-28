use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct CommandEnumRawData {
    pub name: String,
    pub value_indexes: Vec<u32>
}

impl CommandEnumRawData {
    pub fn new(name: String, value_indexes: Vec<u32>) -> CommandEnumRawData {
        CommandEnumRawData { name, value_indexes }
    }

    pub fn read(stream: &mut Stream) -> CommandEnumRawData {
        let name = PacketSerializer::get_string(stream);
        let size = stream.get_var_u32();
        let mut value_indexes = Vec::with_capacity(size as usize);
        for _ in 0..size {
            value_indexes.push(stream.get_u32_le());
        }

        CommandEnumRawData { name, value_indexes }
    }

    pub fn write(&self, stream: &mut Stream) {
        PacketSerializer::put_string(stream, self.name.clone());
        stream.put_var_u32(self.value_indexes.len() as u32);
        for value_index in &self.value_indexes {
            stream.put_u32_le(*value_index);
        }
    }
}
