use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct CameraSetInstructionRotation {
    pub pitch: f32,
    pub yaw: f32
}

impl CameraSetInstructionRotation {
    pub fn new(pitch: f32, yaw: f32) -> CameraSetInstructionRotation {
        CameraSetInstructionRotation{ pitch, yaw }
    }

    pub fn read(stream: &mut Stream) -> CameraSetInstructionRotation {
        let pitch = stream.get_f32_le();
        let yaw = stream.get_f32_le();

        CameraSetInstructionRotation{ pitch, yaw }
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_f32_le(self.pitch);
        stream.put_f32_le(self.yaw);
    }
}