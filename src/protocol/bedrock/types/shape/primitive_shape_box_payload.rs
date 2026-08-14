use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct PrimitiveShapeBoxPayload {
    pub box_bound: Vec<f32>,
}

impl PrimitiveShapeBoxPayload {
    pub fn new(box_bound: Vec<f32>) -> PrimitiveShapeBoxPayload {
        PrimitiveShapeBoxPayload { box_bound }
    }

    pub fn read(stream: &mut Reader) -> PrimitiveShapeBoxPayload {
        let box_bound = PacketSerializer::get_vector3(stream);
        PrimitiveShapeBoxPayload { box_bound }
    }

    pub fn write(&self, stream: &mut Writer) {
        PacketSerializer::put_vector3(stream, &self.box_bound);
    }
}
