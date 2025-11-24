use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

#[derive(serde::Serialize, Debug)]
pub struct CameraRotationOption {
    pub value: Vec<f32>,
    pub time: f32
}

impl CameraRotationOption {
    pub fn new(value: Vec<f32>, time: f32) -> CameraRotationOption {
        CameraRotationOption{ value, time }
    }

    pub fn read(stream: &mut Stream) -> CameraRotationOption {
        let value = PacketSerializer::get_vector3(stream);
        let time = stream.get_f32_le();

        CameraRotationOption{ value, time }
    }

    pub fn write(&self, stream: &mut Stream) {
        PacketSerializer::put_vector3(stream, self.value.clone());
        stream.put_f32_le(self.time);
    }
}