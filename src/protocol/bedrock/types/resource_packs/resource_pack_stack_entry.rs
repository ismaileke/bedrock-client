use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

#[derive(serde::Serialize, Debug)]
pub struct ResourcePackStackEntry {
    pub pack_id: String,
    pub version: String,
    pub sub_pack_name: String
}

impl ResourcePackStackEntry {
    pub fn read(stream: &mut Stream) -> ResourcePackStackEntry {
        let pack_id = PacketSerializer::get_string(stream);
        let version = PacketSerializer::get_string(stream);
        let sub_pack_name = PacketSerializer::get_string(stream);

        ResourcePackStackEntry{ pack_id, version, sub_pack_name }
    }
}