use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct RemoveActor {
    pub actor_unique_id: i64,
}

impl Packet for RemoveActor {
    fn id(&self) -> u16 {
        BedrockPacketType::IDRemoveActor.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_actor_unique_id(stream, self.actor_unique_id);
    }

    fn decode(stream: &mut Reader) -> RemoveActor {
        let actor_unique_id = PacketSerializer::get_actor_unique_id(stream);

        RemoveActor { actor_unique_id }
    }
}
