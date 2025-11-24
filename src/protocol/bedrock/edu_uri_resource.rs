use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::types::education_uri_resource::EducationUriResource;

#[derive(serde::Serialize, Debug)]
pub struct EduUriResource {
    pub resource: EducationUriResource
}

pub fn new(resource: EducationUriResource) -> EduUriResource {
    EduUriResource { resource }
}

impl Packet for EduUriResource {
    fn id(&self) -> u16 {
        BedrockPacketType::IDEduUriResource.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        self.resource.write(&mut stream);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> EduUriResource {
        let resource = EducationUriResource::read(stream);

        EduUriResource { resource }
    }

    fn debug(&self) {
        println!("Resource: {:?}", self.resource);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
