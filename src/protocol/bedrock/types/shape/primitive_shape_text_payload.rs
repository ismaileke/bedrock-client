use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::utils::color::Color;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct PrimitiveShapeTextPayload {
    pub text: String,
    pub use_rotation: bool,
    pub background_color: Option<Color>,
    pub depth_test: bool,
    pub show_backface: bool,
    pub show_text_backface: bool
}

impl PrimitiveShapeTextPayload {
    pub fn new(
        text: String,
        use_rotation: bool,
        background_color: Option<Color>,
        depth_test: bool,
        show_backface: bool,
        show_text_backface: bool
    ) -> PrimitiveShapeTextPayload {
        PrimitiveShapeTextPayload { text, use_rotation, background_color, depth_test, show_backface, show_text_backface }
    }

    pub fn read(stream: &mut Reader) -> PrimitiveShapeTextPayload {
        let text = PacketSerializer::get_string(stream);
        let use_rotation = stream.get_bool();
        let background_color = PacketSerializer::read_optional(stream, |s| Color::from_argb(s.get_u32_le()));
        let depth_test = stream.get_bool();
        let show_backface = stream.get_bool();
        let show_text_backface = stream.get_bool();

        PrimitiveShapeTextPayload { text, use_rotation, background_color, depth_test, show_backface, show_text_backface }
    }

    pub fn write(&self, stream: &mut Writer) {
        PacketSerializer::put_string(stream, &self.text);
        stream.put_bool(self.use_rotation);
        PacketSerializer::write_optional(stream, &self.background_color, |s, v| s.put_u32_le(v.to_argb()));
        stream.put_bool(self.depth_test);
        stream.put_bool(self.show_backface);
        stream.put_bool(self.show_text_backface);
    }
}
