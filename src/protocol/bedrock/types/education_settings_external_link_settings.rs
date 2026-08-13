use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct EducationSettingsExternalLinkSettings {
    pub url: String,
    pub display_name: String,
}

impl EducationSettingsExternalLinkSettings {
    pub fn read(stream: &mut Reader) -> EducationSettingsExternalLinkSettings {
        let url = PacketSerializer::get_string(stream);
        let display_name = PacketSerializer::get_string(stream);

        EducationSettingsExternalLinkSettings { url, display_name }
    }

    pub fn write(&self, stream: &mut Writer) {
        PacketSerializer::put_string(stream, self.url.clone());
        PacketSerializer::put_string(stream, self.display_name.clone());
    }
}
