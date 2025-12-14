use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct CameraFadeInstructionTime {
    pub fade_in_time: f32,
    pub stay_time: f32,
    pub fade_out_time: f32,
}

impl CameraFadeInstructionTime {
    pub fn new(fade_in_time: f32, stay_time: f32, fade_out_time: f32) -> CameraFadeInstructionTime {
        CameraFadeInstructionTime {
            fade_in_time,
            stay_time,
            fade_out_time,
        }
    }

    pub fn read(stream: &mut Stream) -> CameraFadeInstructionTime {
        let fade_in_time = stream.get_f32_le();
        let stay_time = stream.get_f32_le();
        let fade_out_time = stream.get_f32_le();

        CameraFadeInstructionTime {
            fade_in_time,
            stay_time,
            fade_out_time,
        }
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_f32_le(self.fade_in_time);
        stream.put_f32_le(self.stay_time);
        stream.put_f32_le(self.fade_out_time);
    }
}
