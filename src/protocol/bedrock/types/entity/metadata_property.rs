use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::cacheable_nbt::CacheableNBT;
use crate::protocol::bedrock::types::entity::entity_metadata_types::EntityMetadataTypes;
use binary_utils::binary::Stream;
use std::fmt::Debug;

#[derive(serde::Serialize, Debug)]
pub enum MetadataProperty {
    Byte(u8),
    Short(i16),
    Int(i32),
    Float(f32),
    String(String),
    CompoundTag(CacheableNBT),
    BlockPos(Vec<i32>),
    Long(i64),
    Vector3f(Vec<f32>),
}

impl MetadataProperty {
    pub fn id(&self) -> u32 {
        match self {
            MetadataProperty::Byte(_) => EntityMetadataTypes::BYTE,
            MetadataProperty::Short(_) => EntityMetadataTypes::SHORT,
            MetadataProperty::Int(_) => EntityMetadataTypes::INT,
            MetadataProperty::Float(_) => EntityMetadataTypes::FLOAT,
            MetadataProperty::String(_) => EntityMetadataTypes::STRING,
            MetadataProperty::CompoundTag(_) => EntityMetadataTypes::COMPOUND_TAG,
            MetadataProperty::BlockPos(_) => EntityMetadataTypes::BLOCK_POS,
            MetadataProperty::Long(_) => EntityMetadataTypes::LONG,
            MetadataProperty::Vector3f(_) => EntityMetadataTypes::VECTOR3F,
        }
    }

    pub fn write(&mut self, stream: &mut Stream) {
        match self {
            MetadataProperty::Byte(v) => stream.put_byte(*v),
            MetadataProperty::Short(v) => stream.put_i16_le(*v),
            MetadataProperty::Int(v) => stream.put_var_i32(*v),
            MetadataProperty::Float(v) => stream.put_f32_le(*v),
            MetadataProperty::String(v) => PacketSerializer::put_string(stream, v.clone()),
            MetadataProperty::CompoundTag(v) => stream.put(v.get_encoded_nbt()),
            MetadataProperty::BlockPos(v) => {
                PacketSerializer::put_signed_block_pos(stream, v.clone())
            }
            MetadataProperty::Long(v) => stream.put_var_i64(*v),
            MetadataProperty::Vector3f(v) => PacketSerializer::put_vector3(stream, v.clone()),
        }
    }
}
