use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::entity::attribute_modifier::AttributeModifier;

#[derive(Debug)]
pub struct UpdateAttribute {
    pub min: f32,
    pub max: f32,
    pub current: f32,
    pub default_min: f32,
    pub default_max: f32,
    pub default: f32,
    pub id: String,
    pub modifiers: Vec<AttributeModifier>
}

pub fn new(min: f32, max: f32, current: f32, default_min: f32, default_max: f32, default: f32, id: String, modifiers: Vec<AttributeModifier>) -> UpdateAttribute {
    UpdateAttribute{ min, max, current, default_min, default_max, default, id, modifiers }
}

impl UpdateAttribute {
    pub fn read(stream: &mut Stream) -> UpdateAttribute {

        let min = stream.get_l_float();
        let max = stream.get_l_float();
        let current = stream.get_l_float();
        let default_min = stream.get_l_float();
        let default_max = stream.get_l_float();
        let default = stream.get_l_float();
        let id = PacketSerializer::get_string(stream);

        let mut modifiers = vec![];
        let modifier_count = stream.get_unsigned_var_int();
        for _ in 0..modifier_count {
            modifiers.push(AttributeModifier::read(stream));
        }

        UpdateAttribute{ min, max, current, default_min, default_max, default, id, modifiers }
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_l_float(self.min);
        stream.put_l_float(self.max);
        stream.put_l_float(self.current);
        stream.put_l_float(self.default_min);
        stream.put_l_float(self.default_max);
        stream.put_l_float(self.default);
        PacketSerializer::put_string(stream, self.id.clone());
        stream.put_unsigned_var_int(self.modifiers.len() as u32);
        for modifier in self.modifiers.iter() {
            modifier.write(stream);
        }
    }
}