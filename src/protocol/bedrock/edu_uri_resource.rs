use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::types::education_uri_resource::EducationUriResource;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct EduUriResource {
    pub resource: EducationUriResource,
}

impl Packet for EduUriResource {
    fn id(&self) -> u16 {
        BedrockPacketType::IDEduUriResource.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        self.resource.write(stream);
    }

    fn decode(stream: &mut Reader) -> EduUriResource {
        let resource = EducationUriResource::read(stream);

        EduUriResource { resource }
    }
}
