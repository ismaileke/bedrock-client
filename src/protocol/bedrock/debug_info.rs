use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct DebugInfo {
    pub actor_unique_id: i64,
    pub data: String,
}

impl Packet for DebugInfo {
    fn id(&self) -> u16 {
        BedrockPacketType::IDDebugInfo.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_actor_unique_id(stream, self.actor_unique_id);
        PacketSerializer::put_string(stream, &self.data);
    }

    fn decode(stream: &mut Reader) -> DebugInfo {
        let actor_unique_id = PacketSerializer::get_actor_unique_id(stream);
        let data = PacketSerializer::get_string(stream);

        DebugInfo { actor_unique_id, data }
    }
}
