use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::camera::camera_fade_instruction::CameraFadeInstruction;
use crate::protocol::bedrock::types::camera::camera_fov_instruction::CameraFovInstruction;
use crate::protocol::bedrock::types::camera::camera_set_instruction::CameraSetInstruction;
use crate::protocol::bedrock::types::camera::camera_target_instruction::CameraTargetInstruction;

pub struct CameraInstruction {
    pub set: Option<CameraSetInstruction>,
    pub clear: Option<bool>,
    pub fade: Option<CameraFadeInstruction>,
    pub target: Option<CameraTargetInstruction>,
    pub remove_target: Option<bool>,
    pub field_of_view: Option<CameraFovInstruction>
}

pub fn new(
    set: Option<CameraSetInstruction>,
    clear: Option<bool>,
    fade: Option<CameraFadeInstruction>,
    target: Option<CameraTargetInstruction>,
    remove_target: Option<bool>,
    field_of_view: Option<CameraFovInstruction>
) -> CameraInstruction {
    CameraInstruction { set, clear, fade, target, remove_target, field_of_view }
}

impl Packet for CameraInstruction {
    fn id(&self) -> u16 {
        BedrockPacketType::IDCameraInstruction.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        PacketSerializer::write_optional(&mut stream, &self.set, |s, v| v.write(s));
        PacketSerializer::write_optional(&mut stream, &self.clear, |s, v| s.put_bool(*v));
        PacketSerializer::write_optional(&mut stream, &self.fade, |s, v| v.write(s));
        PacketSerializer::write_optional(&mut stream, &self.target, |s, v| v.write(s));
        PacketSerializer::write_optional(&mut stream, &self.remove_target, |s, v| s.put_bool(*v));
        PacketSerializer::write_optional(&mut stream, &self.field_of_view, |s, v| v.write(s));

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> CameraInstruction {
        let mut stream = Stream::new(bytes, 0);

        let set = PacketSerializer::read_optional(&mut stream, |s| CameraSetInstruction::read(s));
        let clear = PacketSerializer::read_optional(&mut stream, |s| s.get_bool());
        let fade = PacketSerializer::read_optional(&mut stream, |s| CameraFadeInstruction::read(s));
        let target = PacketSerializer::read_optional(&mut stream, |s| CameraTargetInstruction::read(s));
        let remove_target = PacketSerializer::read_optional(&mut stream, |s| s.get_bool());
        let field_of_view = PacketSerializer::read_optional(&mut stream, |s| CameraFovInstruction::read(s));

        CameraInstruction { set, clear, fade, target, remove_target, field_of_view }
    }

    fn debug(&self) {
        println!("Set: {:?}", self.set);
        println!("Clear: {:?}", self.clear);
        println!("Fade: {:?}", self.fade);
        println!("Target: {:?}", self.target);
        println!("Remove Target: {:?}", self.remove_target);
        println!("Field of view: {:?}", self.field_of_view);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
