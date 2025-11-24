use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::camera::camera_aim_assist_category_priorities::CameraAimAssistCategoryPriorities;

#[derive(serde::Serialize, Debug)]
pub struct CameraAimAssistCategory {
    pub name: String,
    pub priorities: CameraAimAssistCategoryPriorities
}

impl CameraAimAssistCategory {
    pub fn new(name: String, priorities: CameraAimAssistCategoryPriorities) -> CameraAimAssistCategory {
        CameraAimAssistCategory{ name, priorities }
    }

    pub fn read(stream: &mut Stream) -> CameraAimAssistCategory {
        let name = PacketSerializer::get_string(stream);
        let priorities = CameraAimAssistCategoryPriorities::read(stream);

        CameraAimAssistCategory{ name, priorities }
    }

    pub fn write(&self, stream: &mut Stream) {
        PacketSerializer::put_string(stream, self.name.clone());
        self.priorities.write(stream);
    }
}