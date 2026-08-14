use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct DataStoreUpdateValueString {
    pub value: String
}

impl DataStoreUpdateValueString {
    pub fn new(value: String) -> DataStoreUpdateValueString {
        DataStoreUpdateValueString { value }
    }

    pub fn read(stream: &mut Reader) -> DataStoreUpdateValueString {
        let value = PacketSerializer::get_string(stream);

        DataStoreUpdateValueString { value }
    }

    pub fn write(&mut self, stream: &mut Writer) {
        PacketSerializer::put_string(stream, &self.value);
    }
}
