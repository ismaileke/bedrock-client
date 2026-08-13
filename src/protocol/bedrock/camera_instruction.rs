use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::camera::camera_fade_instruction::CameraFadeInstruction;
use crate::protocol::bedrock::types::camera::camera_fov_instruction::CameraFovInstruction;
use crate::protocol::bedrock::types::camera::camera_set_instruction::CameraSetInstruction;
use crate::protocol::bedrock::types::camera::camera_spline_instruction::CameraSplineInstruction;
use crate::protocol::bedrock::types::camera::camera_target_instruction::CameraTargetInstruction;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct CameraInstruction {
    pub set: Option<CameraSetInstruction>,
    pub clear: Option<bool>,
    pub fade: Option<CameraFadeInstruction>,
    pub target: Option<CameraTargetInstruction>,
    pub remove_target: Option<bool>,
    pub field_of_view: Option<CameraFovInstruction>,
    pub spline: Option<CameraSplineInstruction>,
    pub attach_to_entity: Option<i64>,
    pub detach_from_entity: Option<bool>,
}

impl Packet for CameraInstruction {
    fn id(&self) -> u16 {
        BedrockPacketType::IDCameraInstruction.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::write_optional(stream, &self.set, |s, v| v.write(s));
        PacketSerializer::write_optional(stream, &self.clear, |s, v| s.put_bool(*v));
        PacketSerializer::write_optional(stream, &self.fade, |s, v| v.write(s));
        PacketSerializer::write_optional(stream, &self.target, |s, v| v.write(s));
        PacketSerializer::write_optional(stream, &self.remove_target, |s, v| s.put_bool(*v));
        PacketSerializer::write_optional(stream, &self.field_of_view, |s, v| v.write(s));
        PacketSerializer::write_optional(stream, &self.spline, |s, v| v.write(s));
        PacketSerializer::write_optional(stream, &self.attach_to_entity, |s, v| s.put_i64_le(*v));
        PacketSerializer::write_optional(stream, &self.detach_from_entity, |s, v| s.put_bool(*v));
    }

    fn decode(stream: &mut Reader) -> CameraInstruction {
        let set = PacketSerializer::read_optional(stream, |s| CameraSetInstruction::read(s));
        let clear = PacketSerializer::read_optional(stream, |s| s.get_bool());
        let fade = PacketSerializer::read_optional(stream, |s| CameraFadeInstruction::read(s));
        let target = PacketSerializer::read_optional(stream, |s| CameraTargetInstruction::read(s));
        let remove_target = PacketSerializer::read_optional(stream, |s| s.get_bool());
        let field_of_view = PacketSerializer::read_optional(stream, |s| CameraFovInstruction::read(s));
        let spline = PacketSerializer::read_optional(stream, |s| CameraSplineInstruction::read(s));
        let attach_to_entity = PacketSerializer::read_optional(stream, |s| s.get_i64_le());
        let detach_from_entity = PacketSerializer::read_optional(stream, |s| s.get_bool());

        CameraInstruction {
            set,
            clear,
            fade,
            target,
            remove_target,
            field_of_view,
            spline,
            attach_to_entity,
            detach_from_entity,
        }
    }
}
