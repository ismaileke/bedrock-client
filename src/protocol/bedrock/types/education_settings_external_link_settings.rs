use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

#[derive(Debug)]
pub struct EducationSettingsExternalLinkSettings {
    pub url: String,
    pub display_name: String
}

impl EducationSettingsExternalLinkSettings {
    pub fn read(stream: &mut Stream) -> EducationSettingsExternalLinkSettings {
        let url = PacketSerializer::get_string(stream);
        let display_name = PacketSerializer::get_string(stream);

        EducationSettingsExternalLinkSettings{ url, display_name }
    }

    pub fn write(&self, stream: &mut Stream) {
        PacketSerializer::put_string(stream, self.url.clone());
        PacketSerializer::put_string(stream, self.display_name.clone());
    }
}