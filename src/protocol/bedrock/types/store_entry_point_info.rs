use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct StoreEntryPointInfo {
    id: String,
    name: String,
}

impl StoreEntryPointInfo {
    pub fn new(id: String, name: String) -> StoreEntryPointInfo {
        StoreEntryPointInfo { id, name }
    }

    pub fn read(stream: &mut Stream) -> StoreEntryPointInfo {
        let id = PacketSerializer::get_string(stream);
        let name = PacketSerializer::get_string(stream);
        StoreEntryPointInfo { id, name }
    }

    pub fn write(&self, stream: &mut Stream) {
        PacketSerializer::put_string(stream, self.id.clone());
        PacketSerializer::put_string(stream, self.name.clone());
    }
}
