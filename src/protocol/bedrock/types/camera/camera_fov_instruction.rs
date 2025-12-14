use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct CameraFovInstruction {
    pub field_of_view: f32,
    pub ease_time: f32,
    pub ease_type: u8,
    pub clear: bool,
}

impl CameraFovInstruction {
    pub fn new(
        field_of_view: f32,
        ease_time: f32,
        ease_type: u8,
        clear: bool,
    ) -> CameraFovInstruction {
        CameraFovInstruction {
            field_of_view,
            ease_time,
            ease_type,
            clear,
        }
    }

    pub fn read(stream: &mut Stream) -> CameraFovInstruction {
        let field_of_view = stream.get_f32_le();
        let ease_time = stream.get_f32_le();
        let ease_type = stream.get_byte();
        let clear = stream.get_bool();

        CameraFovInstruction {
            field_of_view,
            ease_time,
            ease_type,
            clear,
        }
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_f32_le(self.field_of_view);
        stream.put_f32_le(self.ease_time);
        stream.put_byte(self.ease_type);
        stream.put_bool(self.clear);
    }
}
