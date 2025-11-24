use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

#[derive(serde::Serialize, Debug)]
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

    pub fn write(&mut self, stream: &mut Stream) {
        PacketSerializer::put_string(stream, self.alias.clone());
    }
}
