use binary_utils::binary::{Reader, Writer};
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::attribute::attribute_environment_payload::AttributeEnvironmentPayload;

#[derive(serde::Serialize, Debug)]
pub struct AttributeEnvironment {
    pub name: String,
    pub payload: AttributeEnvironmentPayload
}

impl AttributeEnvironment {
    pub fn new(name: String, payload: AttributeEnvironmentPayload) -> AttributeEnvironment {
        AttributeEnvironment { name, payload }
    }

    pub fn read(stream: &mut Reader) -> AttributeEnvironment {
        let name = PacketSerializer::get_string(stream);
        let payload = AttributeEnvironmentPayload::read(stream);

        AttributeEnvironment { name, payload }
    }

    pub fn write(&self, stream: &mut Writer) {
        PacketSerializer::put_string(stream, &self.name);
        self.payload.write(stream);
    }
}
