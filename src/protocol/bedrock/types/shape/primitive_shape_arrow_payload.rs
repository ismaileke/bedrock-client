use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct PrimitiveShapeArrowPayload  {
    pub line_end_location: Option<Vec<f32>>,
    pub arrow_head_length: Option<f32>,
    pub arrow_head_radius: Option<f32>,
    pub segments: Option<u8>
}

impl PrimitiveShapeArrowPayload {
    pub fn new(
        line_end_location: Option<Vec<f32>>,
        arrow_head_length: Option<f32>,
        arrow_head_radius: Option<f32>,
        segments: Option<u8>
    ) -> PrimitiveShapeArrowPayload {
        PrimitiveShapeArrowPayload { line_end_location, arrow_head_length, arrow_head_radius, segments }
    }

    pub fn read(stream: &mut Reader) -> PrimitiveShapeArrowPayload {
        let line_end_location = PacketSerializer::read_optional(stream, |s| PacketSerializer::get_vector3(s));
        let arrow_head_length = PacketSerializer::read_optional(stream, |s| s.get_f32_le());
        let arrow_head_radius = PacketSerializer::read_optional(stream, |s| s.get_f32_le());
        let segments = PacketSerializer::read_optional(stream, |s| s.get_u8());

        PrimitiveShapeArrowPayload { line_end_location, arrow_head_length, arrow_head_radius, segments }
    }

    pub fn write(&self, stream: &mut Writer) {
        PacketSerializer::write_optional(stream, &self.line_end_location, |s, v| PacketSerializer::put_vector3(s, v));
        PacketSerializer::write_optional(stream, &self.arrow_head_length, |s, v| s.put_f32_le(*v));
        PacketSerializer::write_optional(stream, &self.arrow_head_radius, |s, v| s.put_f32_le(*v));
        PacketSerializer::write_optional(stream, &self.segments, |s, v| s.put_u8(*v));
    }
}
