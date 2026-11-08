use binary_utils::binary::{Reader, Writer};
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

#[derive(serde::Serialize, Debug)]
pub struct AttributeTransitionSettings {
    pub total_transition_ticks: u32,
    pub current_transition_ticks: u32,
    pub ease_type: i32,
    pub clock_name: String
}

impl AttributeTransitionSettings {
    pub fn read(stream: &mut Reader) -> AttributeTransitionSettings {
        let total_transition_ticks = stream.get_var_u32();
        let current_transition_ticks = stream.get_var_u32();
        let ease_type = stream.get_var_i32();
        let clock_name = PacketSerializer::get_string(stream);
        AttributeTransitionSettings {
            total_transition_ticks,
            current_transition_ticks,
            ease_type,
            clock_name
        }
    }

    pub fn write(&self, stream: &mut Writer) {
        stream.put_var_u32(self.total_transition_ticks);
        stream.put_var_u32(self.current_transition_ticks);
        stream.put_var_i32(self.ease_type);
        PacketSerializer::put_string(stream, &self.clock_name);
    }
}
