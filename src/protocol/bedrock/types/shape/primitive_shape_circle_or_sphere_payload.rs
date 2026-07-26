use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct PrimitiveShapeCircleOrSpherePayload {
    pub segments: u8,
}

impl PrimitiveShapeCircleOrSpherePayload {
    pub fn new(segments: u8) -> PrimitiveShapeCircleOrSpherePayload {
        PrimitiveShapeCircleOrSpherePayload { segments }
    }

    pub fn read(stream: &mut Stream) -> PrimitiveShapeCircleOrSpherePayload {
        let segments = stream.get_byte();
        PrimitiveShapeCircleOrSpherePayload { segments }
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_byte(self.segments);
    }
}
