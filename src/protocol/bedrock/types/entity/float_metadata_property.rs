use binary_utils::binary::Stream;
use crate::protocol::bedrock::types::entity::entity_metadata_types::EntityMetadataTypes;
use crate::protocol::bedrock::types::entity::metadata_property::MetadataProperty;

#[derive(Debug)]
pub struct FloatMetadataProperty {
    value: f32
}

impl FloatMetadataProperty {
    pub fn new(value: f32) -> FloatMetadataProperty {
        FloatMetadataProperty{ value }
    }

    pub fn read(stream: &mut Stream) -> FloatMetadataProperty {
        FloatMetadataProperty{ value: stream.get_l_float() }
    }
}
impl MetadataProperty for FloatMetadataProperty {
    fn id(&self) -> u32 {
        EntityMetadataTypes::FLOAT
    }

    fn write(&mut self, stream: &mut Stream) {
        stream.put_l_float(self.value);
    }
}