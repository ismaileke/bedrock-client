use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct AttributeValueColorString {
    pub value: String
}

impl AttributeValueColorString {

    pub fn new(value: String) -> AttributeValueColorString {
        AttributeValueColorString { value }
    }

    pub fn read(stream: &mut Stream) -> AttributeValueColorString {
        let value = PacketSerializer::get_string(stream);

        AttributeValueColorString { value }
    }

    pub fn write(&self, stream: &mut Stream) {
        PacketSerializer::put_string(stream, self.value.clone());
    }
}
