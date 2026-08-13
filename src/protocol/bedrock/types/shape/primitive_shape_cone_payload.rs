use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct PrimitiveShapeConePayload {
    pub radii: Vec<f32>,
    pub height: f32,
    pub segments: u8
}

impl PrimitiveShapeConePayload {
    pub fn new(radii: Vec<f32>, height: f32, segments: u8) -> PrimitiveShapeConePayload {
        PrimitiveShapeConePayload { radii, height, segments }
    }

    pub fn read(stream: &mut Reader) -> PrimitiveShapeConePayload {
        let radii = PacketSerializer::get_vector2(stream);
        let height = stream.get_f32_le();
        let segments = stream.get_u8();
        PrimitiveShapeConePayload { radii, height, segments }
    }

    pub fn write(&self, stream: &mut Writer) {
        PacketSerializer::put_vector2(stream, self.radii.clone());
        stream.put_f32_le(self.height);
        stream.put_u8(self.segments);
    }
}
