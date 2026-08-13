use binary_utils::binary::{Reader, Writer};
/*use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::attribute_value::AttributeValue;*/
use crate::utils::color::Color;

#[derive(serde::Serialize, Debug)]
pub struct MapInfoRequestPacketClientPixel {
    pub color: Color,
    pub x: u16,
    pub y: u16
}

impl MapInfoRequestPacketClientPixel {
    const Y_INDEX_MULTIPLIER: u16 = 128;

    pub fn new(color: Color, x: u16, y: u16) -> MapInfoRequestPacketClientPixel {
        MapInfoRequestPacketClientPixel { color, x, y }
    }

    pub fn read(stream: &mut Reader) -> MapInfoRequestPacketClientPixel {
        let color = stream.get_u32_le();
        let index = stream.get_u16_le();

        let x = index % Self::Y_INDEX_MULTIPLIER;
        let y = index / Self::Y_INDEX_MULTIPLIER;

        MapInfoRequestPacketClientPixel { color: Color::from_rgba(color), x, y }
    }

    pub fn write(&self, stream: &mut Writer) {
        stream.put_u32_le(self.color.to_rgba());
        stream.put_u16_le(self.x + (self.y * Self::Y_INDEX_MULTIPLIER));
    }
}
