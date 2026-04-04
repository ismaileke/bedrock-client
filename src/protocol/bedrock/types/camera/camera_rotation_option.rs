use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct CameraRotationOption {
    pub value: Vec<f32>,
    pub time: f32,
    pub ease_type: String,
}

impl CameraRotationOption {
    pub fn new(value: Vec<f32>, time: f32, ease_type: String) -> CameraRotationOption {
        CameraRotationOption { value, time, ease_type }
    }

    pub fn read(stream: &mut Stream) -> CameraRotationOption {
        let value = PacketSerializer::get_vector3(stream);
        let time = stream.get_f32_le();
        let ease_type = PacketSerializer::get_string(stream);

        CameraRotationOption { value, time, ease_type }
    }

    pub fn write(&self, stream: &mut Stream) {
        PacketSerializer::put_vector3(stream, self.value.clone());
        stream.put_f32_le(self.time);
        PacketSerializer::put_string(stream, self.ease_type.clone());
    }
}
