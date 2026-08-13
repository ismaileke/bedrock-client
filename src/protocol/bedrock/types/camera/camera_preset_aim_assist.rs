use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct CameraPresetAimAssist {
    pub preset_id: Option<String>,
    pub target_mode: Option<u8>, //see types/camera_aim_assist_target_mode.rs
    pub view_angle: Option<Vec<f32>>,
    pub distance: Option<f32>,
}

impl CameraPresetAimAssist {
    pub fn read(stream: &mut Reader) -> CameraPresetAimAssist {
        let preset_id =
            PacketSerializer::read_optional(stream, |s| PacketSerializer::get_string(s));
        let target_mode = PacketSerializer::read_optional(stream, |s| s.get_u8());
        let view_angle =
            PacketSerializer::read_optional(stream, |s| PacketSerializer::get_vector2(s));
        let distance = PacketSerializer::read_optional(stream, |s| s.get_f32_le());

        CameraPresetAimAssist {
            preset_id,
            target_mode,
            view_angle,
            distance,
        }
    }

    pub fn write(&self, stream: &mut Writer) {
        PacketSerializer::write_optional(stream, &self.preset_id, |s, v| {
            PacketSerializer::put_string(s, v.clone())
        });
        PacketSerializer::write_optional(stream, &self.target_mode, |s, v| s.put_u8(*v));
        PacketSerializer::write_optional(stream, &self.view_angle, |s, v| {
            PacketSerializer::put_vector2(s, v.clone())
        });
        PacketSerializer::write_optional(stream, &self.distance, |s, v| s.put_f32_le(*v));
    }
}
