use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::recipe::item_descriptor::ItemDescriptor;
use crate::protocol::bedrock::types::recipe::item_descriptor_type::ItemDescriptorType;

#[derive(Debug)]
pub struct ComplexAliasItemDescriptor {
    alias: String
}

impl ComplexAliasItemDescriptor {
    pub fn new(alias: String) -> ComplexAliasItemDescriptor {
        ComplexAliasItemDescriptor{ alias }
    }

    pub fn read(stream: &mut Stream) -> ComplexAliasItemDescriptor {
        let alias = PacketSerializer::get_string(stream);

        ComplexAliasItemDescriptor{ alias, }
    }
}

impl ItemDescriptor for ComplexAliasItemDescriptor {
    fn get_type_id(&self) -> u8 {
        ItemDescriptorType::COMPLEX_ALIAS
    }

    fn write(&mut self, stream: &mut Stream) {
        PacketSerializer::put_string(stream, self.alias.clone());
    }
}


