use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct AttributeValueColorString {
    pub value: String
}

impl AttributeValueColorString {

    pub fn new(value: String) -> AttributeValueColorString {
        AttributeValueColorString { value }
    }

    pub fn read(stream: &mut Reader) -> AttributeValueColorString {
        let value = PacketSerializer::get_string(stream);

        AttributeValueColorString { value }
    }

    pub fn write(&self, stream: &mut Writer) {
        PacketSerializer::put_string(stream, self.value.clone());
    }
}
