use binary_utils::binary::Stream;
use crate::protocol::bedrock::types::entity::entity_metadata_types::EntityMetadataTypes;
use crate::protocol::bedrock::types::entity::metadata_property::MetadataProperty;

#[derive(Debug)]
pub struct LongMetadataProperty {
    value: i64
}

impl LongMetadataProperty {
    pub fn new(value: i64) -> LongMetadataProperty {
        LongMetadataProperty{ value }
    }

    pub fn read(stream: &mut Stream) -> LongMetadataProperty {
        LongMetadataProperty{ value: stream.get_var_long() }
    }
}
impl MetadataProperty for LongMetadataProperty {
    fn id(&self) -> u32 {
        EntityMetadataTypes::LONG
    }

    fn write(&mut self, stream: &mut Stream) {
        stream.put_var_long(self.value);
    }
}