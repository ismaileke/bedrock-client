use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct CameraFadeInstructionColor {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

impl CameraFadeInstructionColor {
    pub fn new(red: f32, green: f32, blue: f32) -> CameraFadeInstructionColor {
        CameraFadeInstructionColor { red, green, blue }
    }

    pub fn read(stream: &mut Stream) -> CameraFadeInstructionColor {
        let red = stream.get_f32_le();
        let green = stream.get_f32_le();
        let blue = stream.get_f32_le();

        CameraFadeInstructionColor { red, green, blue }
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_f32_le(self.red);
        stream.put_f32_le(self.green);
        stream.put_f32_le(self.blue);
    }
}
