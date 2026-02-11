use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct CameraAimAssistActorPriorityData {
    pub preset_index: i32,
    pub category_index: i32,
    pub actor_index: i32,
    pub priority: i32,
}

impl CameraAimAssistActorPriorityData {
    pub fn new(preset_index: i32, category_index: i32, actor_index: i32, priority: i32,) -> CameraAimAssistActorPriorityData {
        CameraAimAssistActorPriorityData { preset_index, category_index, actor_index, priority }
    }

    pub fn read(stream: &mut Stream) -> CameraAimAssistActorPriorityData {
        let preset_index = stream.get_i32_le();
        let category_index = stream.get_i32_le();
        let actor_index = stream.get_i32_le();
        let priority = stream.get_i32_le();

        CameraAimAssistActorPriorityData { preset_index, category_index, actor_index, priority }
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_i32_le(self.preset_index);
        stream.put_i32_le(self.category_index);
        stream.put_i32_le(self.actor_index);
        stream.put_i32_le(self.priority);
    }
}
