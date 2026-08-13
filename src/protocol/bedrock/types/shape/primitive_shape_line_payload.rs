use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct PrimitiveShapeLinePayload {
    pub line_end_location: Vec<f32>
}

impl PrimitiveShapeLinePayload {
    pub fn new(line_end_location: Vec<f32>) -> PrimitiveShapeLinePayload {
        PrimitiveShapeLinePayload { line_end_location }
    }

    pub fn read(stream: &mut Reader) -> PrimitiveShapeLinePayload {
        let line_end_location = PacketSerializer::get_vector3(stream);
        PrimitiveShapeLinePayload { line_end_location }
    }

    pub fn write(&self, stream: &mut Writer) {
        PacketSerializer::put_vector3(stream, self.line_end_location.clone());
    }
}
