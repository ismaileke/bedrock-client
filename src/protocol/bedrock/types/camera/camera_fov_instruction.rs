use binary_utils::binary::Stream;

#[derive(Debug)]
pub struct CameraFovInstruction {
    field_of_view: f32,
    ease_time: f32,
    ease_type: u8,
    clear: bool
}

impl CameraFovInstruction {
    pub fn new(field_of_view: f32, ease_time: f32, ease_type: u8, clear: bool) -> CameraFovInstruction {
        CameraFovInstruction{ field_of_view, ease_time, ease_type, clear }
    }

    pub fn read(stream: &mut Stream) -> CameraFovInstruction {
        let field_of_view = stream.get_l_float();
        let ease_time = stream.get_l_float();
        let ease_type = stream.get_byte();
        let clear = stream.get_bool();

        CameraFovInstruction{ field_of_view, ease_time, ease_type, clear }
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_l_float(self.field_of_view);
        stream.put_l_float(self.ease_time);
        stream.put_byte(self.ease_type);
        stream.put_bool(self.clear);
    }
}