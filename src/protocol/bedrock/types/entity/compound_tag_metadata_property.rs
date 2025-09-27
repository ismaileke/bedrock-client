use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::cacheable_nbt::CacheableNBT;
use crate::protocol::bedrock::types::entity::entity_metadata_types::EntityMetadataTypes;
use crate::protocol::bedrock::types::entity::metadata_property::MetadataProperty;

#[derive(Debug)]
pub struct CompoundTagMetadataProperty {
    value: CacheableNBT
}

impl CompoundTagMetadataProperty {
    pub fn new(value: CacheableNBT) -> CompoundTagMetadataProperty {
        CompoundTagMetadataProperty{ value }
    }

    pub fn read(stream: &mut Stream) -> CompoundTagMetadataProperty {
        CompoundTagMetadataProperty{ value: CacheableNBT::new(Box::new(PacketSerializer::get_nbt_compound_root(stream))) }
    }
}
impl MetadataProperty for CompoundTagMetadataProperty {
    fn id(&self) -> u32 {
        EntityMetadataTypes::COMPOUND_TAG
    }

    fn write(&mut self, stream: &mut Stream) {
        stream.put(self.value.get_encoded_nbt());
    }
}