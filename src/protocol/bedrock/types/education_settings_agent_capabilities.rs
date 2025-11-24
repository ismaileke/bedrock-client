use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

#[derive(serde::Serialize, Debug)]
pub struct EducationSettingsAgentCapabilities {
    pub can_modify_blocks: Option<bool>
}

impl EducationSettingsAgentCapabilities {
    pub fn read(stream: &mut Stream) -> EducationSettingsAgentCapabilities {
        let can_modify_blocks = PacketSerializer::read_optional(stream, |s| s.get_bool());

        EducationSettingsAgentCapabilities{ can_modify_blocks }
    }

    pub fn write(&self, stream: &mut Stream) {
        PacketSerializer::write_optional(stream, &self.can_modify_blocks, |s, v| s.put_bool(*v));
    }
}