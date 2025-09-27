use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::entity::entity_metadata_types::EntityMetadataTypes;
use crate::protocol::bedrock::types::entity::metadata_property::MetadataProperty;

#[derive(Debug)]
pub struct Vec3MetadataProperty {
    value: Vec<f32>
}

impl Vec3MetadataProperty {
    pub fn new(value: Vec<f32>) -> Vec3MetadataProperty {
        Vec3MetadataProperty{ value }
    }

    pub fn read(stream: &mut Stream) -> Vec3MetadataProperty {
        Vec3MetadataProperty{ value: PacketSerializer::get_vector3(stream) }
    }
}
impl MetadataProperty for Vec3MetadataProperty {
    fn id(&self) -> u32 {
        EntityMetadataTypes::VECTOR3F
    }

    fn write(&mut self, stream: &mut Stream) {
        PacketSerializer::put_vector3(stream, self.value.clone());
    }
}