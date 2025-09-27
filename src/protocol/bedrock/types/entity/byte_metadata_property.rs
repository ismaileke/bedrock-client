use binary_utils::binary::Stream;
use crate::protocol::bedrock::types::entity::entity_metadata_types::EntityMetadataTypes;
use crate::protocol::bedrock::types::entity::metadata_property::MetadataProperty;

#[derive(Debug)]
pub struct ByteMetadataProperty {
    value: u8
}

impl ByteMetadataProperty {
    pub fn new(value: u8) -> ByteMetadataProperty {
        ByteMetadataProperty{ value }
    }

    pub fn read(stream: &mut Stream) -> ByteMetadataProperty {
        ByteMetadataProperty{ value: stream.get_byte() } //as i8?? idk
    }
}
impl MetadataProperty for ByteMetadataProperty {
    fn id(&self) -> u32 {
        EntityMetadataTypes::BYTE
    }

    fn write(&mut self, stream: &mut Stream) {
        stream.put_byte(self.value);
    }
}