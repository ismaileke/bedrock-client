use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct MolangItemDescriptor {
    molang_expression: String,
    version: u8,
}

impl MolangItemDescriptor {
    pub fn new(molang_expression: String, version: u8) -> MolangItemDescriptor {
        MolangItemDescriptor {
            molang_expression,
            version,
        }
    }

    pub fn read(stream: &mut Reader) -> MolangItemDescriptor {
        let molang_expression = PacketSerializer::get_string(stream);
        let version = stream.get_u8();

        MolangItemDescriptor {
            molang_expression,
            version,
        }
    }

    pub fn write(&mut self, stream: &mut Writer) {
        PacketSerializer::put_string(stream, self.molang_expression.clone());
        stream.put_u8(self.version);
    }
}
