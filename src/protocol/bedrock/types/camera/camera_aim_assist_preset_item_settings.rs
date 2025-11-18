use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

#[derive(Debug)]
pub struct CameraAimAssistPresetItemSettings {
    pub item_identifier: String,
    pub category_name: String
}

impl CameraAimAssistPresetItemSettings {
    pub fn new(item_identifier: String, category_name: String) -> CameraAimAssistPresetItemSettings {
        CameraAimAssistPresetItemSettings{ item_identifier, category_name }
    }

    pub fn read(stream: &mut Stream) -> CameraAimAssistPresetItemSettings {
        let item_identifier = PacketSerializer::get_string(stream);
        let category_name = PacketSerializer::get_string(stream);

        CameraAimAssistPresetItemSettings{ item_identifier, category_name }
    }

    pub fn write(&self, stream: &mut Stream) {
        PacketSerializer::put_string(stream, self.item_identifier.clone());
        PacketSerializer::put_string(stream, self.category_name.clone());
    }
}