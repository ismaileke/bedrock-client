use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct CameraProgressOption {
    pub value: f32,
    pub time: f32,
    pub ease_type: u32, // see types/camera/camera_set_instruction_ease_type.rs
}

impl CameraProgressOption {
    pub fn new(value: f32, time: f32, ease_type: u32) -> CameraProgressOption {
        CameraProgressOption { value, time, ease_type }
    }

    pub fn read(stream: &mut Stream) -> CameraProgressOption {
        let value = stream.get_f32_le();
        let time = stream.get_f32_le();
        let ease_type = stream.get_u32_le();

        CameraProgressOption { value, time, ease_type }
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_f32_le(self.value);
        stream.put_f32_le(self.time);
        stream.put_u32_le(self.ease_type);
    }
}
