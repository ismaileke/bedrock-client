use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct PrimitiveShapePyramidPayload {
    pub width: f32,
    pub depth: Option<f32>,
    pub height: f32
}

impl PrimitiveShapePyramidPayload {
    pub fn new(width: f32, depth: Option<f32>, height: f32) -> PrimitiveShapePyramidPayload {
        PrimitiveShapePyramidPayload { width, depth, height }
    }

    pub fn read(stream: &mut Stream) -> PrimitiveShapePyramidPayload {
        let width = stream.get_f32_le();
        let depth = PacketSerializer::read_optional(stream, |s| s.get_f32_le());
        let height = stream.get_f32_le();
        PrimitiveShapePyramidPayload { width, depth, height }
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_f32_le(self.width);
        PacketSerializer::write_optional(stream, &self.depth, |s, v| s.put_f32_le(*v));
        stream.put_f32_le(self.height);
    }
}
