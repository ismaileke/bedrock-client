use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};
use crate::protocol::bedrock::types::camera::camera_spline_instruction::CameraSplineInstruction;

#[derive(serde::Serialize, Debug)]
pub struct CameraSplineDefinition {
    pub name: String,
    pub instruction: CameraSplineInstruction,
}

impl CameraSplineDefinition {
    pub fn new(name: String, instruction: CameraSplineInstruction) -> CameraSplineDefinition {
        CameraSplineDefinition { name, instruction }
    }

    pub fn read(stream: &mut Reader) -> CameraSplineDefinition {
        let name = PacketSerializer::get_string(stream);
        let instruction = CameraSplineInstruction::read(stream);

        CameraSplineDefinition { name, instruction }
    }

    pub fn write(&self, stream: &mut Writer) {
        PacketSerializer::put_string(stream, &self.name);
        self.instruction.write(stream);
    }
}
