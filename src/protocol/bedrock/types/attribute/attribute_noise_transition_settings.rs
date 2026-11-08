use binary_utils::binary::{Reader, Writer};
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::attribute::noise_alignment::NoiseAlignment;

#[derive(serde::Serialize, Debug)]
pub struct AttributeNoiseTransitionSettings {
    pub total_transition_ticks: u32,
    pub current_transition_ticks: u32,
    pub ease_type: i32,
    pub clock_name: String,
    pub local_transition_ticks: u32,
    pub noise_name: String,
    pub noise_alignment: NoiseAlignment
}

impl AttributeNoiseTransitionSettings {
    pub fn read(stream: &mut Reader) -> AttributeNoiseTransitionSettings {
        let total_transition_ticks = stream.get_var_u32();
        let current_transition_ticks = stream.get_var_u32();
        let ease_type = stream.get_var_i32();
        let clock_name = PacketSerializer::get_string(stream);
        let local_transition_ticks = stream.get_var_u32();
        let noise_name = PacketSerializer::get_string(stream);
        let noise_alignment = NoiseAlignment::read(stream);
        AttributeNoiseTransitionSettings {
            total_transition_ticks,
            current_transition_ticks,
            ease_type,
            clock_name,
            local_transition_ticks,
            noise_name,
            noise_alignment
        }
    }

    pub fn write(&self, stream: &mut Writer) {
        stream.put_var_u32(self.total_transition_ticks);
        stream.put_var_u32(self.current_transition_ticks);
        stream.put_var_i32(self.ease_type);
        PacketSerializer::put_string(stream, &self.clock_name);
        stream.put_var_u32(self.local_transition_ticks);
        PacketSerializer::put_string(stream, &self.noise_name);
        self.noise_alignment.write(stream);
    }
}
