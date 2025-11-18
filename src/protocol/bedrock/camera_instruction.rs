use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::camera::camera_fade_instruction::CameraFadeInstruction;
use crate::protocol::bedrock::types::camera::camera_fov_instruction::CameraFovInstruction;
use crate::protocol::bedrock::types::camera::camera_set_instruction::CameraSetInstruction;
use crate::protocol::bedrock::types::camera::camera_spline_instruction::CameraSplineInstruction;
use crate::protocol::bedrock::types::camera::camera_target_instruction::CameraTargetInstruction;

pub struct CameraInstruction {
    pub set: Option<CameraSetInstruction>,
    pub clear: Option<bool>,
    pub fade: Option<CameraFadeInstruction>,
    pub target: Option<CameraTargetInstruction>,
    pub remove_target: Option<bool>,
    pub field_of_view: Option<CameraFovInstruction>,
    pub spline: Option<CameraSplineInstruction>,
    pub attach_to_entity: Option<i64>,
    pub detach_from_entity: Option<bool>
}

pub fn new(
    set: Option<CameraSetInstruction>,
    clear: Option<bool>,
    fade: Option<CameraFadeInstruction>,
    target: Option<CameraTargetInstruction>,
    remove_target: Option<bool>,
    field_of_view: Option<CameraFovInstruction>,
    spline: Option<CameraSplineInstruction>,
    attach_to_entity: Option<i64>,
    detach_from_entity: Option<bool>
) -> CameraInstruction {
    CameraInstruction { set, clear, fade, target, remove_target, field_of_view, spline, attach_to_entity, detach_from_entity }
}

impl Packet for CameraInstruction {
    fn id(&self) -> u16 {
        BedrockPacketType::IDCameraInstruction.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        PacketSerializer::write_optional(&mut stream, &self.set, |s, v| v.write(s));
        PacketSerializer::write_optional(&mut stream, &self.clear, |s, v| s.put_bool(*v));
        PacketSerializer::write_optional(&mut stream, &self.fade, |s, v| v.write(s));
        PacketSerializer::write_optional(&mut stream, &self.target, |s, v| v.write(s));
        PacketSerializer::write_optional(&mut stream, &self.remove_target, |s, v| s.put_bool(*v));
        PacketSerializer::write_optional(&mut stream, &self.field_of_view, |s, v| v.write(s));
        PacketSerializer::write_optional(&mut stream, &self.spline, |s, v| v.write(s));
        PacketSerializer::write_optional(&mut stream, &self.attach_to_entity, |s, v| s.put_i64_le(*v));
        PacketSerializer::write_optional(&mut stream, &self.detach_from_entity, |s, v| s.put_bool(*v));

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(bytes: Vec<u8>) -> CameraInstruction {
        let mut stream = Stream::new(bytes, 0);

        let set = PacketSerializer::read_optional(&mut stream, |s| CameraSetInstruction::read(s));
        let clear = PacketSerializer::read_optional(&mut stream, |s| s.get_bool());
        let fade = PacketSerializer::read_optional(&mut stream, |s| CameraFadeInstruction::read(s));
        let target = PacketSerializer::read_optional(&mut stream, |s| CameraTargetInstruction::read(s));
        let remove_target = PacketSerializer::read_optional(&mut stream, |s| s.get_bool());
        let field_of_view = PacketSerializer::read_optional(&mut stream, |s| CameraFovInstruction::read(s));
        let spline = PacketSerializer::read_optional(&mut stream, |s| CameraSplineInstruction::read(s));
        let attach_to_entity = PacketSerializer::read_optional(&mut stream, |s| s.get_i64_le());
        let detach_from_entity = PacketSerializer::read_optional(&mut stream, |s| s.get_bool());

        CameraInstruction { set, clear, fade, target, remove_target, field_of_view, spline, attach_to_entity, detach_from_entity }
    }

    fn debug(&self) {
        println!("Set: {:?}", self.set);
        println!("Clear: {:?}", self.clear);
        println!("Fade: {:?}", self.fade);
        println!("Target: {:?}", self.target);
        println!("Remove Target: {:?}", self.remove_target);
        println!("Field of view: {:?}", self.field_of_view);
        println!("Spline: {:?}", self.spline);
        println!("Attach to entity: {:?}", self.attach_to_entity);
        println!("Detach from entity: {:?}", self.detach_from_entity);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
