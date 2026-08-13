use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct PrimitiveShapeEllipsoidPayload {
    pub radii: Vec<f32>,
    pub segments_per_axis: u8
}

impl PrimitiveShapeEllipsoidPayload {
    pub fn new(radii: Vec<f32>, segments_per_axis: u8) -> PrimitiveShapeEllipsoidPayload {
        PrimitiveShapeEllipsoidPayload { radii, segments_per_axis }
    }

    pub fn read(stream: &mut Reader) -> PrimitiveShapeEllipsoidPayload {
        let radii = PacketSerializer::get_vector3(stream);
        let segments_per_axis = stream.get_u8();
        PrimitiveShapeEllipsoidPayload { radii, segments_per_axis }
    }

    pub fn write(&self, stream: &mut Writer) {
        PacketSerializer::put_vector3(stream, self.radii.clone());
        stream.put_u8(self.segments_per_axis);
    }
}
