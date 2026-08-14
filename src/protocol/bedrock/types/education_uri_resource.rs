use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct EducationUriResource {
    pub button_name: String,
    pub link_uri: String,
}

impl EducationUriResource {
    pub fn read(stream: &mut Reader) -> EducationUriResource {
        let button_name = PacketSerializer::get_string(stream);
        let link_uri = PacketSerializer::get_string(stream);

        EducationUriResource {
            button_name,
            link_uri,
        }
    }

    pub fn write(&self, stream: &mut Writer) {
        PacketSerializer::put_string(stream, &self.button_name);
        PacketSerializer::put_string(stream, &self.link_uri);
    }
}
