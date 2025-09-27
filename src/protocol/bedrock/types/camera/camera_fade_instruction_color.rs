use binary_utils::binary::Stream;

#[derive(Debug)]
pub struct CameraFadeInstructionColor {
    red: f32,
    green: f32,
    blue: f32
}

impl CameraFadeInstructionColor {
    pub fn new(red: f32, green: f32, blue: f32) -> CameraFadeInstructionColor {
        CameraFadeInstructionColor{ red, green, blue }
    }

    pub fn read(stream: &mut Stream) -> CameraFadeInstructionColor {
        let red = stream.get_l_float();
        let green = stream.get_l_float();
        let blue = stream.get_l_float();

        CameraFadeInstructionColor{ red, green, blue }
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_l_float(self.red);
        stream.put_l_float(self.green);
        stream.put_l_float(self.blue);
    }
}