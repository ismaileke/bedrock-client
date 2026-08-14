use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct PrimitiveShapeCylinderPayload {
    pub radius_x: Vec<f32>,
    pub radius_z: Vec<f32>,
    pub height: f32,
    pub segments: u8
}

impl PrimitiveShapeCylinderPayload {
    pub fn new(radius_x: Vec<f32>, radius_z: Vec<f32>, height: f32, segments: u8) -> PrimitiveShapeCylinderPayload {
        PrimitiveShapeCylinderPayload { radius_x, radius_z, height, segments }
    }

    pub fn read(stream: &mut Reader) -> PrimitiveShapeCylinderPayload {
        let radius_x = PacketSerializer::get_vector2(stream);
        let radius_z = PacketSerializer::get_vector2(stream);
        let height = stream.get_f32_le();
        let segments = stream.get_u8();
        PrimitiveShapeCylinderPayload { radius_x, radius_z, height, segments }
    }

    pub fn write(&self, stream: &mut Writer) {
        PacketSerializer::put_vector2(stream, &self.radius_x);
        PacketSerializer::put_vector2(stream, &self.radius_z);
        stream.put_f32_le(self.height);
        stream.put_u8(self.segments);
    }
}
