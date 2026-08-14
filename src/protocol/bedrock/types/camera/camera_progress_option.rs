use binary_utils::binary::{Reader, Writer};
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

#[derive(serde::Serialize, Debug)]
pub struct CameraProgressOption {
    pub value: f32,
    pub time: f32,
    pub ease_type: String, // see types/camera/camera_set_instruction_ease_type.rs
}

impl CameraProgressOption {
    pub fn new(value: f32, time: f32, ease_type: String) -> CameraProgressOption {
        CameraProgressOption { value, time, ease_type }
    }

    pub fn read(stream: &mut Reader) -> CameraProgressOption {
        let value = stream.get_f32_le();
        let time = stream.get_f32_le();
        let ease_type = PacketSerializer::get_string(stream);

        CameraProgressOption { value, time, ease_type }
    }

    pub fn write(&self, stream: &mut Writer) {
        stream.put_f32_le(self.value);
        stream.put_f32_le(self.time);
        PacketSerializer::put_string(stream, &self.ease_type);
    }
}
