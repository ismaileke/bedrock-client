use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

#[derive(Debug)]
pub struct CameraAimAssistCategoryBlockPriority {
    identifier: String,
    priority: u32
}

impl CameraAimAssistCategoryBlockPriority {
    pub fn new(identifier: String, priority: u32) -> CameraAimAssistCategoryBlockPriority {
        CameraAimAssistCategoryBlockPriority{ identifier, priority }
    }

    pub fn read(stream: &mut Stream) -> CameraAimAssistCategoryBlockPriority {
        let identifier = PacketSerializer::get_string(stream);
        let priority = stream.get_l_int();

        CameraAimAssistCategoryBlockPriority{ identifier, priority }
    }

    pub fn write(&self, stream: &mut Stream) {
        PacketSerializer::put_string(stream, self.identifier.clone());
        stream.put_l_int(self.priority);
    }
}