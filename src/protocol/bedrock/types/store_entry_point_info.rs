use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct StoreEntryPointInfo {
    pub id: String,
    pub name: String,
}

impl StoreEntryPointInfo {
    pub fn new(id: String, name: String) -> StoreEntryPointInfo {
        StoreEntryPointInfo { id, name }
    }

    pub fn read(stream: &mut Reader) -> StoreEntryPointInfo {
        let id = PacketSerializer::get_string(stream);
        let name = PacketSerializer::get_string(stream);
        StoreEntryPointInfo { id, name }
    }

    pub fn write(&self, stream: &mut Writer) {
        PacketSerializer::put_string(stream, &self.id);
        PacketSerializer::put_string(stream, &self.name);
    }
}
