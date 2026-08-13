use binary_utils::binary::{Reader, Writer};
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

#[derive(serde::Serialize, Debug)]
pub struct CameraFovInstruction {
    pub field_of_view: f32,
    pub ease_time: f32,
    pub ease_type: String,
    pub clear: bool,
}

impl CameraFovInstruction {
    pub fn new(field_of_view: f32, ease_time: f32, ease_type: String, clear: bool) -> CameraFovInstruction {
        CameraFovInstruction { field_of_view, ease_time, ease_type, clear }
    }

    pub fn read(stream: &mut Reader) -> CameraFovInstruction {
        let field_of_view = stream.get_f32_le();
        let ease_time = stream.get_f32_le();
        let ease_type = PacketSerializer::get_string(stream);
        let clear = stream.get_bool();

        CameraFovInstruction { field_of_view, ease_time, ease_type, clear }
    }

    pub fn write(&self, stream: &mut Writer) {
        stream.put_f32_le(self.field_of_view);
        stream.put_f32_le(self.ease_time);
        PacketSerializer::put_string(stream, self.ease_type.clone());
        stream.put_bool(self.clear);
    }
}
