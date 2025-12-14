use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct CameraSetInstructionEase {
    pub ease_type: u8, //see types/camera_set_instruction_ease_type.rs
    pub duration: f32,
}

impl CameraSetInstructionEase {
    pub fn new(ease_type: u8, duration: f32) -> CameraSetInstructionEase {
        CameraSetInstructionEase {
            ease_type,
            duration,
        }
    }

    pub fn read(stream: &mut Stream) -> CameraSetInstructionEase {
        let ease_type = stream.get_byte();
        let duration = stream.get_f32_le();

        CameraSetInstructionEase {
            ease_type,
            duration,
        }
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_byte(self.ease_type);
        stream.put_f32_le(self.duration);
    }
}
