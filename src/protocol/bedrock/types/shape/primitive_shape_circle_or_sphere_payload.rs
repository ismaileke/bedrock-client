use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct PrimitiveShapeCircleOrSpherePayload {
    pub segments: u8,
}

impl PrimitiveShapeCircleOrSpherePayload {
    pub fn new(segments: u8) -> PrimitiveShapeCircleOrSpherePayload {
        PrimitiveShapeCircleOrSpherePayload { segments }
    }

    pub fn read(stream: &mut Reader) -> PrimitiveShapeCircleOrSpherePayload {
        let segments = stream.get_u8();
        PrimitiveShapeCircleOrSpherePayload { segments }
    }

    pub fn write(&self, stream: &mut Writer) {
        stream.put_u8(self.segments);
    }
}
