use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::education_settings_agent_capabilities::EducationSettingsAgentCapabilities;
use crate::protocol::bedrock::types::education_settings_external_link_settings::EducationSettingsExternalLinkSettings;

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
    pub link_settings: Option<EducationSettingsExternalLinkSettings>
}

pub fn new(
    code_builder_default_uri: String,
    code_builder_title: String,
    can_resize_code_builder: bool,
    disable_legacy_title_bar: bool,
    post_process_filter: String,
    screenshot_border_resource_path: String,
    agent_capabilities: Option<EducationSettingsAgentCapabilities>,
    code_builder_override_uri: Option<String>,
    has_quiz: bool,
    link_settings: Option<EducationSettingsExternalLinkSettings>
) -> EducationSettings {
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
        link_settings
    }
}

impl Packet for EducationSettings {
    fn id(&self) -> u16 {
        BedrockPacketType::IDEducationSettings.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        PacketSerializer::put_string(&mut stream, self.code_builder_default_uri.clone());
        PacketSerializer::put_string(&mut stream, self.code_builder_title.clone());
        stream.put_bool(self.can_resize_code_builder);
        stream.put_bool(self.disable_legacy_title_bar);
        PacketSerializer::put_string(&mut stream, self.post_process_filter.clone());
        PacketSerializer::put_string(&mut stream, self.screenshot_border_resource_path.clone());
        PacketSerializer::write_optional(&mut stream, &self.agent_capabilities, |s, v| v.write(s));
        PacketSerializer::write_optional(&mut stream, &self.code_builder_override_uri, |s, v| PacketSerializer::put_string(s, v.clone()));
        stream.put_bool(self.has_quiz);
        PacketSerializer::write_optional(&mut stream, &self.link_settings, |s, v| v.write(s));

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> EducationSettings {
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
            link_settings
        }
    }

    fn debug(&self) {
        println!("Code Builder Default URI: {}", self.code_builder_default_uri);
        println!("Code Builder Title: {}", self.code_builder_title);
        println!("Can Resize Code Builder: {}", self.can_resize_code_builder);
        println!("Disable Legacy Title Bar: {}", self.disable_legacy_title_bar);
        println!("Post Process Filter: {}", self.post_process_filter);
        println!("Screenshot Border Resource Path: {}", self.screenshot_border_resource_path);
        println!("Agent Capabilities: {:?}", self.agent_capabilities);
        println!("Code Builder Override URI: {:?}", self.code_builder_override_uri);
        println!("Has Quiz: {}", self.has_quiz);
        println!("Link Settings: {:?}", self.link_settings);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
