use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct CameraAimAssistCategoryPriority {
    pub identifier: String,
    pub priority: i32,
}

impl CameraAimAssistCategoryPriority {
    pub fn new(identifier: String, priority: i32) -> CameraAimAssistCategoryPriority {
        CameraAimAssistCategoryPriority {
            identifier,
            priority,
        }
    }

    pub fn read(stream: &mut Stream) -> CameraAimAssistCategoryPriority {
        let identifier = PacketSerializer::get_string(stream);
        let priority = stream.get_i32_le();

        CameraAimAssistCategoryPriority {
            identifier,
            priority,
        }
    }

    pub fn write(&self, stream: &mut Stream) {
        PacketSerializer::put_string(stream, self.identifier.clone());
        stream.put_i32_le(self.priority);
    }
}
