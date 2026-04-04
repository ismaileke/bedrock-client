use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

#[derive(serde::Serialize, Debug)]
pub struct AttributeLayerSettingsWeightString {
    pub value: String,
}

impl AttributeLayerSettingsWeightString {

    pub fn new(value: String) -> AttributeLayerSettingsWeightString {
        AttributeLayerSettingsWeightString { value }
    }

    pub fn read(stream: &mut Stream) -> AttributeLayerSettingsWeightString {
        AttributeLayerSettingsWeightString { value: PacketSerializer::get_string(stream) }
    }

    pub fn write(&self, stream: &mut Stream) {
        PacketSerializer::put_string(stream, self.value.clone());
    }
}
