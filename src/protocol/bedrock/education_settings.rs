use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::education_settings_agent_capabilities::EducationSettingsAgentCapabilities;
use crate::protocol::bedrock::types::education_settings_external_link_settings::EducationSettingsExternalLinkSettings;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct EducationSettings {
    pub code_builder_default_uri: String,
    pub code_builder_title: String,
    pub can_resize_code_builder: bool,
    pub disable_legacy_title_bar: bool,
    pub post_process_filter: String,
    pub screenshot_border_resource_path: String,
    pub agent_capabilities: Option<EducationSettingsAgentCapabilities>,
    pub code_builder_override_uri: Option<String>,
    pub has_quiz: bool,
    pub link_settings: Option<EducationSettingsExternalLinkSettings>,
}

impl Packet for EducationSettings {
    fn id(&self) -> u16 {
        BedrockPacketType::IDEducationSettings.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_string(stream, self.code_builder_default_uri.clone());
        PacketSerializer::put_string(stream, self.code_builder_title.clone());
        stream.put_bool(self.can_resize_code_builder);
        stream.put_bool(self.disable_legacy_title_bar);
        PacketSerializer::put_string(stream, self.post_process_filter.clone());
        PacketSerializer::put_string(stream, self.screenshot_border_resource_path.clone());
        PacketSerializer::write_optional(stream, &self.agent_capabilities, |s, v| v.write(s));
        PacketSerializer::write_optional(stream, &self.code_builder_override_uri, |s, v| {
            PacketSerializer::put_string(s, v.clone())
        });
        stream.put_bool(self.has_quiz);
        PacketSerializer::write_optional(stream, &self.link_settings, |s, v| v.write(s));
    }

    fn decode(stream: &mut Reader) -> EducationSettings {
        let code_builder_default_uri = PacketSerializer::get_string(stream);
        let code_builder_title = PacketSerializer::get_string(stream);
        let can_resize_code_builder = stream.get_bool();
        let disable_legacy_title_bar = stream.get_bool();
        let post_process_filter = PacketSerializer::get_string(stream);
        let screenshot_border_resource_path = PacketSerializer::get_string(stream);
        let agent_capabilities = PacketSerializer::read_optional(stream, |s| EducationSettingsAgentCapabilities::read(s));
        let code_builder_override_uri = PacketSerializer::read_optional(stream, |s| PacketSerializer::get_string(s));
        let has_quiz = stream.get_bool();
        let link_settings = PacketSerializer::read_optional(stream, |s| EducationSettingsExternalLinkSettings::read(s));

        EducationSettings {
            code_builder_default_uri,
            code_builder_title,
            can_resize_code_builder,
            disable_legacy_title_bar,
            post_process_filter,
            screenshot_border_resource_path,
            agent_capabilities,
            code_builder_override_uri,
            has_quiz,
            link_settings,
        }
    }
}
