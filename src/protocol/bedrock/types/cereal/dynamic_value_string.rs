use binary_utils::binary::{Reader, Writer};
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

#[derive(serde::Serialize, Debug)]
pub struct DynamicValueString {
    pub value: String
}

impl DynamicValueString {

    pub fn new(value: String) -> DynamicValueString {
        DynamicValueString { value }
    }

    pub fn read(stream: &mut Reader) -> DynamicValueString {
        let value = PacketSerializer::get_string(stream);

        DynamicValueString { value }
    }

    pub fn write(&self, stream: &mut Writer) {
        PacketSerializer::put_string(stream, &self.value);
    }
}
