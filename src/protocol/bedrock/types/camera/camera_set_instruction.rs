use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::camera::camera_set_instruction_ease::CameraSetInstructionEase;
use crate::protocol::bedrock::types::camera::camera_set_instruction_rotation::CameraSetInstructionRotation;

#[derive(serde::Serialize, Debug)]
pub struct CameraSetInstruction {
    pub preset: Option<u32>,
    pub ease: Option<CameraSetInstructionEase>,
    pub camera_position: Option<Vec<f32>>,
    pub rotation: Option<CameraSetInstructionRotation>,
    pub facing_position: Option<Vec<f32>>,
    pub view_offset: Option<Vec<f32>>,
    pub entity_offset: Option<Vec<f32>>,
    pub default: Option<bool>,
    pub ignore_starting_values_component: Option<bool>
}

impl CameraSetInstruction {

    pub fn read(stream: &mut Stream) -> CameraSetInstruction {
        let preset = PacketSerializer::read_optional(stream, |s| s.get_u32_le());
        let ease = PacketSerializer::read_optional(stream, |s| CameraSetInstructionEase::read(s));
        let camera_position = PacketSerializer::read_optional(stream, |s| PacketSerializer::get_vector3(s));
        let rotation = PacketSerializer::read_optional(stream, |s| CameraSetInstructionRotation::read(s));
        let facing_position = PacketSerializer::read_optional(stream, |s| PacketSerializer::get_vector3(s));
        let view_offset = PacketSerializer::read_optional(stream, |s| PacketSerializer::get_vector2(s));
        let entity_offset = PacketSerializer::read_optional(stream, |s| PacketSerializer::get_vector3(s));
        let default = PacketSerializer::read_optional(stream, |s| s.get_bool());
        let ignore_starting_values_component = PacketSerializer::read_optional(stream, |s| s.get_bool());

        CameraSetInstruction{ preset, ease, camera_position, rotation, facing_position, view_offset, entity_offset, default, ignore_starting_values_component }
    }

    pub fn write(&self, stream: &mut Stream) {
        PacketSerializer::write_optional(stream, &self.preset, |s, v| s.put_u32_le(*v));
        PacketSerializer::write_optional(stream, &self.ease, |s, v| v.write(s));
        PacketSerializer::write_optional(stream, &self.camera_position, |s, v| PacketSerializer::put_vector3(s, v.clone()));
        PacketSerializer::write_optional(stream, &self.rotation, |s, v| v.write(s));
        PacketSerializer::write_optional(stream, &self.facing_position, |s, v| PacketSerializer::put_vector3(s, v.clone()));
        PacketSerializer::write_optional(stream, &self.view_offset, |s, v| PacketSerializer::put_vector2(s, v.clone()));
        PacketSerializer::write_optional(stream, &self.entity_offset, |s, v| PacketSerializer::put_vector3(s, v.clone()));
        PacketSerializer::write_optional(stream, &self.default, |s, v| s.put_bool(*v));
        PacketSerializer::write_optional(stream, &self.ignore_starting_values_component, |s, v| s.put_bool(*v));
    }
}