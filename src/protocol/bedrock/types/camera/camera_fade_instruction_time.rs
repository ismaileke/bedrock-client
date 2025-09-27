use binary_utils::binary::Stream;

#[derive(Debug)]
pub struct CameraFadeInstructionTime {
    fade_in_time: f32,
    stay_time: f32,
    fade_out_time: f32
}

impl CameraFadeInstructionTime {
    pub fn new(fade_in_time: f32, stay_time: f32, fade_out_time: f32) -> CameraFadeInstructionTime {
        CameraFadeInstructionTime{ fade_in_time, stay_time, fade_out_time }
    }

    pub fn read(stream: &mut Stream) -> CameraFadeInstructionTime {
        let fade_in_time = stream.get_l_float();
        let stay_time = stream.get_l_float();
        let fade_out_time = stream.get_l_float();

        CameraFadeInstructionTime{ fade_in_time, stay_time, fade_out_time }
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_l_float(self.fade_in_time);
        stream.put_l_float(self.stay_time);
        stream.put_l_float(self.fade_out_time);
    }
}