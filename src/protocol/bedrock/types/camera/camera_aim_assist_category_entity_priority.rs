use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

#[derive(Debug)]
pub struct CameraAimAssistCategoryEntityPriority {
    identifier: String,
    priority: u32
}

impl CameraAimAssistCategoryEntityPriority {
    pub fn new(identifier: String, priority: u32) -> CameraAimAssistCategoryEntityPriority {
        CameraAimAssistCategoryEntityPriority{ identifier, priority }
    }

    pub fn read(stream: &mut Stream) -> CameraAimAssistCategoryEntityPriority {
        let identifier = PacketSerializer::get_string(stream);
        let priority = stream.get_l_int();

        CameraAimAssistCategoryEntityPriority{ identifier, priority }
    }

    pub fn write(&self, stream: &mut Stream) {
        PacketSerializer::put_string(stream, self.identifier.clone());
        stream.put_l_int(self.priority);
    }
}