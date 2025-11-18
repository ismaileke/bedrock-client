use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::camera::camera_fade_instruction_color::CameraFadeInstructionColor;
use crate::protocol::bedrock::types::camera::camera_fade_instruction_time::CameraFadeInstructionTime;

#[derive(Debug)]
pub struct CameraFadeInstruction {
    pub time: Option<CameraFadeInstructionTime>,
    pub color: Option<CameraFadeInstructionColor>
}

impl CameraFadeInstruction {
    pub fn new(time: Option<CameraFadeInstructionTime>, color: Option<CameraFadeInstructionColor>) -> CameraFadeInstruction {
        CameraFadeInstruction{ time, color }
    }

    pub fn read(stream: &mut Stream) -> CameraFadeInstruction {
        let time = PacketSerializer::read_optional(stream, |s| CameraFadeInstructionTime::read(s));
        let color = PacketSerializer::read_optional(stream, |s| CameraFadeInstructionColor::read(s));

        CameraFadeInstruction{ time, color }
    }

    pub fn write(&self, stream: &mut Stream) {
        PacketSerializer::write_optional(stream, &self.time, |s, v| v.write(s));
        PacketSerializer::write_optional(stream, &self.color, |s, v| v.write(s));
    }
}