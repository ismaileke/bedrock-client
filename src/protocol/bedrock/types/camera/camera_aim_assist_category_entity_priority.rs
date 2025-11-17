use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

#[derive(Debug)]
pub struct CameraAimAssistCategoryEntityPriority {
    pub identifier: String,
    pub priority: i32
}

impl CameraAimAssistCategoryEntityPriority {
    pub fn new(identifier: String, priority: i32) -> CameraAimAssistCategoryEntityPriority {
        CameraAimAssistCategoryEntityPriority{ identifier, priority }
    }

    pub fn read(stream: &mut Stream) -> CameraAimAssistCategoryEntityPriority {
        let identifier = PacketSerializer::get_string(stream);
        let priority = stream.get_i32_le();

        CameraAimAssistCategoryEntityPriority{ identifier, priority }
    }

    pub fn write(&self, stream: &mut Stream) {
        PacketSerializer::put_string(stream, self.identifier.clone());
        stream.put_i32_le(self.priority);
    }
}