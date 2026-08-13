use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct MapCreateLockedCopy {
    pub original_map_id: i64,
    pub new_map_id: i64,
}

impl Packet for MapCreateLockedCopy {
    fn id(&self) -> u16 {
        BedrockPacketType::IDMapCreateLockedCopy.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_actor_unique_id(stream, self.original_map_id);
        PacketSerializer::put_actor_unique_id(stream, self.new_map_id);
    }

    fn decode(stream: &mut Reader) -> MapCreateLockedCopy {
        let original_map_id = PacketSerializer::get_actor_unique_id(stream);
        let new_map_id = PacketSerializer::get_actor_unique_id(stream);

        MapCreateLockedCopy { original_map_id, new_map_id }
    }
}
