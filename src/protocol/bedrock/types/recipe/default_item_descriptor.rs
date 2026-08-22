use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug, Clone)]
pub struct DefaultItemDescriptor {
    name: String,
    metadata_value: i32,
}

impl DefaultItemDescriptor {
    pub fn new(name: String, metadata_value: i32) -> DefaultItemDescriptor {
        DefaultItemDescriptor { name, metadata_value }
    }

    pub fn read(stream: &mut Reader) -> DefaultItemDescriptor {
        let name = PacketSerializer::get_string(stream);
        let metadata_value = stream.get_var_i32();

        DefaultItemDescriptor { name, metadata_value }
    }

    pub fn write(&self, stream: &mut Writer) {
        PacketSerializer::put_string(stream, &self.name);
        stream.put_var_i32(self.metadata_value);
    }
}
