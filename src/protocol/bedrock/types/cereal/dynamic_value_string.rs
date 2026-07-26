use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

#[derive(serde::Serialize, Debug)]
pub struct DynamicValueString {
    pub value: String
}

impl DynamicValueString {

    pub fn new(value: String) -> DynamicValueString {
        DynamicValueString { value }
    }

    pub fn read(stream: &mut Stream) -> DynamicValueString {
        let value = PacketSerializer::get_string(stream);

        DynamicValueString { value }
    }

    pub fn write(&self, stream: &mut Stream) {
        PacketSerializer::put_string(stream, self.value.clone());
    }
}
