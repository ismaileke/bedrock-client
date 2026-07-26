use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::attribute_value::AttributeValue;

#[derive(serde::Serialize, Debug)]
pub struct AttributeEnvironment {
    pub name: String,
    pub from_attribute: Option<AttributeValue>,
    pub attribute: AttributeValue,
    pub to_attribute: Option<AttributeValue>,
    pub current_transition_ticks: u32,
    pub total_transition_ticks: u32,
    pub ease_type: String,
    pub local_transition_ticks: u32,
    pub noise_transition: bool,
}

impl AttributeEnvironment {
    pub fn new(
        name: String,
        from_attribute: Option<AttributeValue>,
        attribute: AttributeValue,
        to_attribute: Option<AttributeValue>,
        current_transition_ticks: u32,
        total_transition_ticks: u32,
        ease_type: String,
        local_transition_ticks: u32,
        noise_transition: bool,
    ) -> AttributeEnvironment {
        AttributeEnvironment {
            name,
            from_attribute,
            attribute,
            to_attribute,
            current_transition_ticks,
            total_transition_ticks,
            ease_type,
            local_transition_ticks,
            noise_transition,
        }
    }

    pub fn read(stream: &mut Stream) -> AttributeEnvironment {
        let name = PacketSerializer::get_string(stream);
        let from_attribute = PacketSerializer::read_optional(stream, |s| AttributeValue::read(s));
        let attribute = AttributeValue::read(stream);
        let to_attribute = PacketSerializer::read_optional(stream, |s| AttributeValue::read(s));
        let current_transition_ticks = stream.get_u32_le();
        let total_transition_ticks = stream.get_u32_le();
        let ease_type = PacketSerializer::get_string(stream);
        let local_transition_ticks = stream.get_u32_le();
        let noise_transition = stream.get_bool();

        AttributeEnvironment {
            name,
            from_attribute,
            attribute,
            to_attribute,
            current_transition_ticks,
            total_transition_ticks,
            ease_type,
            local_transition_ticks,
            noise_transition,
        }
    }

    pub fn write(&self, stream: &mut Stream) {
        PacketSerializer::put_string(stream, self.name.clone());
        PacketSerializer::write_optional(stream, &self.from_attribute, |s, v| v.write(s));
        self.attribute.write(stream);
        PacketSerializer::write_optional(stream, &self.to_attribute, |s, v| v.write(s));
        stream.put_u32_le(self.current_transition_ticks);
        stream.put_u32_le(self.total_transition_ticks);
        PacketSerializer::put_string(stream, self.ease_type.clone());
        stream.put_u32_le(self.local_transition_ticks);
        stream.put_bool(self.noise_transition);
    }
}
