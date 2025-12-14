use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct DataStoreValueString {
    pub value: String,
}

impl DataStoreValueString {
    pub fn new(value: String) -> DataStoreValueString {
        DataStoreValueString { value }
    }

    pub fn read(stream: &mut Stream) -> DataStoreValueString {
        let value = PacketSerializer::get_string(stream);

        DataStoreValueString { value }
    }

    pub fn write(&mut self, stream: &mut Stream) {
        PacketSerializer::put_string(stream, self.value.clone());
    }
}
