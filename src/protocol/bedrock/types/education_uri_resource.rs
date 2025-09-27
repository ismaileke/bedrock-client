use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

#[derive(Debug)]
pub struct EducationUriResource {
    pub button_name: String,
    pub link_uri: String
}

impl EducationUriResource {
    pub fn read(stream: &mut Stream) -> EducationUriResource {
        let button_name = PacketSerializer::get_string(stream);
        let link_uri = PacketSerializer::get_string(stream);

        EducationUriResource{ button_name, link_uri }
    }

    pub fn write(&self, stream: &mut Stream) {
        PacketSerializer::put_string(stream, self.button_name.clone());
        PacketSerializer::put_string(stream, self.link_uri.clone());
    }
}