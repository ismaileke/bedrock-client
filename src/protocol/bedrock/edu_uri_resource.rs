use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::types::education_uri_resource::EducationUriResource;

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
        stream.put_unsigned_var_int(self.id() as u32);

        self.resource.write(&mut stream);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> EduUriResource {
        let mut stream = Stream::new(bytes, 0);

        let resource = EducationUriResource::read(&mut stream);

        EduUriResource { resource }
    }

    fn debug(&self) {
        println!("Resource: {:?}", self.resource);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
