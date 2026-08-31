use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug, Clone)]
pub struct MolangItemDescriptor {
    pub molang_expression: String,
    pub version: i16,
}

impl MolangItemDescriptor {
    pub fn new(molang_expression: String, version: i16) -> MolangItemDescriptor {
        MolangItemDescriptor { molang_expression, version }
    }

    pub fn read(stream: &mut Reader) -> MolangItemDescriptor {
        let molang_expression = PacketSerializer::get_string(stream);
        let version = stream.get_i16_le();

        MolangItemDescriptor { molang_expression, version }
    }

    pub fn write(&self, stream: &mut Writer) {
        PacketSerializer::put_string(stream, &self.molang_expression);
        stream.put_i16_le(self.version);
    }
}
