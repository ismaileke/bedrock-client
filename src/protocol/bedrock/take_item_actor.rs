use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct TakeItemActor {
    pub item_actor_runtime_id: u64,
    pub taker_actor_runtime_id: u64,
}

impl Packet for TakeItemActor {
    fn id(&self) -> u16 {
        BedrockPacketType::IDTakeItemActor.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_actor_runtime_id(stream, self.item_actor_runtime_id);
        PacketSerializer::put_actor_runtime_id(stream, self.taker_actor_runtime_id);
    }

    fn decode(stream: &mut Reader) -> TakeItemActor {
        let item_actor_runtime_id = PacketSerializer::get_actor_runtime_id(stream);
        let taker_actor_runtime_id = PacketSerializer::get_actor_runtime_id(stream);

        TakeItemActor { item_actor_runtime_id, taker_actor_runtime_id }
    }
}
