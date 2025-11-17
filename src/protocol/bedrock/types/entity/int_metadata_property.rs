use binary_utils::binary::Stream;
use crate::protocol::bedrock::types::entity::entity_metadata_types::EntityMetadataTypes;
use crate::protocol::bedrock::types::entity::metadata_property::MetadataProperty;

#[derive(Debug)]
pub struct IntMetadataProperty {
    value: i32
}

impl IntMetadataProperty {
    pub fn new(value: i32) -> IntMetadataProperty {
        IntMetadataProperty{ value }
    }

    pub fn read(stream: &mut Stream) -> IntMetadataProperty {
        IntMetadataProperty{ value: stream.get_var_i32() }
    }
}
impl MetadataProperty for IntMetadataProperty {
    fn id(&self) -> u32 {
        EntityMetadataTypes::INT
    }

    fn write(&mut self, stream: &mut Stream) {
        stream.put_var_i32(self.value);
    }
}