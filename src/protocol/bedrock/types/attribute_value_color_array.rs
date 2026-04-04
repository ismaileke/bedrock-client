use crate::utils::color::Color;
use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct AttributeValueColorArray {
    pub value: Color
}

impl AttributeValueColorArray {

    pub fn new(value: Color) -> AttributeValueColorArray {
        AttributeValueColorArray { value }
    }

    pub fn read(stream: &mut Stream) -> AttributeValueColorArray {
        let r = stream.get_u32_le();
        let g = stream.get_u32_le();
        let b = stream.get_u32_le();
        let a = stream.get_u32_le();

        AttributeValueColorArray { value: Color::new(r, g, b, a) }
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_u32_le(self.value.r() as u32);
        stream.put_u32_le(self.value.g() as u32);
        stream.put_u32_le(self.value.b() as u32);
        stream.put_u32_le(self.value.a() as u32);
    }
}
