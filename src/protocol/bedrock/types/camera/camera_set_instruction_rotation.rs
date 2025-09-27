use binary_utils::binary::Stream;

#[derive(Debug)]
pub struct CameraSetInstructionRotation {
    pitch: f32,
    yaw: f32
}

impl CameraSetInstructionRotation {
    pub fn new(pitch: f32, yaw: f32) -> CameraSetInstructionRotation {
        CameraSetInstructionRotation{ pitch, yaw }
    }

    pub fn read(stream: &mut Stream) -> CameraSetInstructionRotation {
        let pitch = stream.get_l_float();
        let yaw = stream.get_l_float();

        CameraSetInstructionRotation{ pitch, yaw }
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_l_float(self.pitch);
        stream.put_l_float(self.yaw);
    }
}