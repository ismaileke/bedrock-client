use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct CommandSoftEnum {
    pub name: String,
    pub values: Vec<String>
}

impl CommandSoftEnum {
    pub fn new(name: String, values: Vec<String>) -> CommandSoftEnum {
        CommandSoftEnum { name, values }
    }

    pub fn read(stream: &mut Reader) -> CommandSoftEnum {
        let name = PacketSerializer::get_string(stream);
        let size = stream.get_var_u32();
        let mut values = Vec::with_capacity(size as usize);
        for _ in 0..size {
            values.push(PacketSerializer::get_string(stream));
        }

        CommandSoftEnum { name, values }
    }

    pub fn write(&self, stream: &mut Writer) {
        PacketSerializer::put_string(stream, self.name.clone());
        stream.put_var_u32(self.values.len() as u32);
        for value_index in &self.values {
            PacketSerializer::put_string(stream, value_index.clone());
        }
    }
}
