use binary_utils::binary::Stream;
use crate::protocol::bedrock::types::entity::entity_metadata_types::EntityMetadataTypes;
use crate::protocol::bedrock::types::entity::metadata_property::MetadataProperty;

#[derive(Debug)]
pub struct ShortMetadataProperty {
    value: u16
}

impl ShortMetadataProperty {
    pub fn new(value: u16) -> ShortMetadataProperty {
        ShortMetadataProperty{ value }
    }

    pub fn read(stream: &mut Stream) -> ShortMetadataProperty {
        ShortMetadataProperty{ value: stream.get_l_short() } // signed_l_short?? idk
    }
}
impl MetadataProperty for ShortMetadataProperty {
    fn id(&self) -> u32 {
        EntityMetadataTypes::SHORT
    }

    fn write(&mut self, stream: &mut Stream) {
        stream.put_l_short(self.value);
    }
}