use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

#[derive(Debug)]
pub struct CameraAimAssistCategoryBlockPriority {
    pub identifier: String,
    pub priority: i32
}

impl CameraAimAssistCategoryBlockPriority {
    pub fn new(identifier: String, priority: i32) -> CameraAimAssistCategoryBlockPriority {
        CameraAimAssistCategoryBlockPriority{ identifier, priority }
    }

    pub fn read(stream: &mut Stream) -> CameraAimAssistCategoryBlockPriority {
        let identifier = PacketSerializer::get_string(stream);
        let priority = stream.get_i32_le();

        CameraAimAssistCategoryBlockPriority{ identifier, priority }
    }

    pub fn write(&self, stream: &mut Stream) {
        PacketSerializer::put_string(stream, self.identifier.clone());
        stream.put_i32_le(self.priority);
    }
}