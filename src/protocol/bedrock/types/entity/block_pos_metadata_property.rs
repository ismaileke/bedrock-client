use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::entity::entity_metadata_types::EntityMetadataTypes;
use crate::protocol::bedrock::types::entity::metadata_property::MetadataProperty;

#[derive(Debug)]
pub struct BlockPosMetadataProperty {
    value: Vec<i32>
}

impl BlockPosMetadataProperty {
    pub fn new(value: Vec<i32>) -> BlockPosMetadataProperty {
        BlockPosMetadataProperty{ value }
    }

    pub fn read(stream: &mut Stream) -> BlockPosMetadataProperty {
        BlockPosMetadataProperty{ value: PacketSerializer::get_signed_block_pos(stream) }
    }
}
impl MetadataProperty for BlockPosMetadataProperty {
    fn id(&self) -> u32 {
        EntityMetadataTypes::BLOCK_POS
    }

    fn write(&mut self, stream: &mut Stream) {
        PacketSerializer::put_signed_block_pos(stream, self.value.clone());
    }
}