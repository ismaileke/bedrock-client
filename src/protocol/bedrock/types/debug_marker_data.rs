use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::utils::color::Color;
use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct DebugMarkerData {
    pub text: String,
    pub position: Vec<f32>,
    pub color: Color,
    pub duration_millis: u64,
}

impl DebugMarkerData {
    pub fn new(
        text: String,
        position: Vec<f32>,
        color: Color,
        duration_millis: u64,
    ) -> DebugMarkerData {
        DebugMarkerData {
            text,
            position,
            color,
            duration_millis,
        }
    }

    pub fn read(stream: &mut Stream) -> DebugMarkerData {
        let text = PacketSerializer::get_string(stream);
        let position = PacketSerializer::get_vector3(stream);
        let color = Color::from_argb(stream.get_u32_le());
        let duration_millis = stream.get_u64_le();

        DebugMarkerData {
            text,
            position,
            color,
            duration_millis,
        }
    }

    pub fn write(&self, stream: &mut Stream) {
        PacketSerializer::put_string(stream, self.text.clone());
        PacketSerializer::put_vector3(stream, self.position.clone());
        stream.put_u32_le(self.color.to_argb());
        stream.put_u64_le(self.duration_millis);
    }
}
