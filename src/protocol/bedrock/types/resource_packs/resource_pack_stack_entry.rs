use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct ResourcePackStackEntry {
    pub pack_id: String,
    pub version: String,
    pub sub_pack_name: String,
}

impl ResourcePackStackEntry {
    pub fn read(stream: &mut Reader) -> ResourcePackStackEntry {
        let pack_id = PacketSerializer::get_string(stream);
        let version = PacketSerializer::get_string(stream);
        let sub_pack_name = PacketSerializer::get_string(stream);

        ResourcePackStackEntry { pack_id, version, sub_pack_name }
    }

    pub fn write(&self, stream: &mut Writer) {
        PacketSerializer::put_string(stream, &self.pack_id);
        PacketSerializer::put_string(stream, &self.version);
        PacketSerializer::put_string(stream, &self.sub_pack_name);
    }
}
