use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct PrimitiveShapeEllipsoidPayload {
    pub radii: Vec<f32>,
    pub segments_per_axis: u8
}

impl PrimitiveShapeEllipsoidPayload {
    pub fn new(radii: Vec<f32>, segments_per_axis: u8) -> PrimitiveShapeEllipsoidPayload {
        PrimitiveShapeEllipsoidPayload { radii, segments_per_axis }
    }

    pub fn read(stream: &mut Stream) -> PrimitiveShapeEllipsoidPayload {
        let radii = PacketSerializer::get_vector3(stream);
        let segments_per_axis = stream.get_byte();
        PrimitiveShapeEllipsoidPayload { radii, segments_per_axis }
    }

    pub fn write(&self, stream: &mut Stream) {
        PacketSerializer::put_vector3(stream, self.radii.clone());
        stream.put_byte(self.segments_per_axis);
    }
}
