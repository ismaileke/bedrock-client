use binary_utils::binary::Stream;

#[derive(Debug)]
pub struct CameraSetInstructionEase {
    ease_type: u8, //see types/camera_set_instruction_ease_type.rs
    duration: f32
}

impl CameraSetInstructionEase {
    pub fn new(ease_type: u8, duration: f32) -> CameraSetInstructionEase {
        CameraSetInstructionEase{ ease_type, duration }
    }

    pub fn read(stream: &mut Stream) -> CameraSetInstructionEase {
        let ease_type = stream.get_byte();
        let duration = stream.get_l_float();

        CameraSetInstructionEase{ ease_type, duration }
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_byte(self.ease_type);
        stream.put_l_float(self.duration);
    }
}