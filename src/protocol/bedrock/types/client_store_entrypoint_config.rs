use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct ClientStoreEntrypointConfig {
    store_id: String,
    store_name: String,
}

impl ClientStoreEntrypointConfig {
    pub fn new(store_id: String, store_name: String) -> ClientStoreEntrypointConfig {
        ClientStoreEntrypointConfig { store_id, store_name }
    }

    pub fn read(stream: &mut Reader) -> ClientStoreEntrypointConfig {
        let store_id = PacketSerializer::get_string(stream);
        let store_name = PacketSerializer::get_string(stream);
        ClientStoreEntrypointConfig { store_id, store_name }
    }

    pub fn write(&self, stream: &mut Writer) {
        PacketSerializer::put_string(stream, self.store_id.clone());
        PacketSerializer::put_string(stream, self.store_name.clone());
    }
}
