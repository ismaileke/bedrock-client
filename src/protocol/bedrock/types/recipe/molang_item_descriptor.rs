use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::recipe::item_descriptor::ItemDescriptor;
use crate::protocol::bedrock::types::recipe::item_descriptor_type::ItemDescriptorType;

#[derive(Debug)]
pub struct MolangItemDescriptor {
    molang_expression: String,
    version: u8
}

impl MolangItemDescriptor {
    pub fn new(molang_expression: String, version: u8) -> MolangItemDescriptor {
        MolangItemDescriptor{ molang_expression, version }
    }

    pub fn read(stream: &mut Stream) -> MolangItemDescriptor {
        let molang_expression = PacketSerializer::get_string(stream);
        let version = stream.get_byte();

        MolangItemDescriptor{ molang_expression, version }
    }
}

impl ItemDescriptor for MolangItemDescriptor {
    fn get_type_id(&self) -> u8 {
        ItemDescriptorType::MOLANG
    }

    fn write(&mut self, stream: &mut Stream) {
        PacketSerializer::put_string(stream, self.molang_expression.clone());
        stream.put_byte(self.version);
    }
}


