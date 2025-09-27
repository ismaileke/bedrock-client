use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::entity::entity_metadata_types::EntityMetadataTypes;
use crate::protocol::bedrock::types::entity::metadata_property::MetadataProperty;

#[derive(Debug)]
pub struct StringMetadataProperty {
    value: String
}

impl StringMetadataProperty {
    pub fn new(value: String) -> StringMetadataProperty {
        StringMetadataProperty{ value }
    }

    pub fn read(stream: &mut Stream) -> StringMetadataProperty {
        StringMetadataProperty{ value: PacketSerializer::get_string(stream) }
    }
}
impl MetadataProperty for StringMetadataProperty {
    fn id(&self) -> u32 {
        EntityMetadataTypes::STRING
    }

    fn write(&mut self, stream: &mut Stream) {
        PacketSerializer::put_string(stream, self.value.clone())
    }
}