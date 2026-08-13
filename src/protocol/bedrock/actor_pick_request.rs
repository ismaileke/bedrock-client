use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct ActorPickRequest {
    pub actor_unique_id: i64,
    pub add_user_data: bool,
    pub hotbar_slot: u8,
}

impl Packet for ActorPickRequest {
    fn id(&self) -> u16 {
        BedrockPacketType::IDActorPickRequest.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_actor_unique_id(stream, self.actor_unique_id);
        stream.put_bool(self.add_user_data);
        stream.put_u8(self.hotbar_slot);
    }

    fn decode(stream: &mut Reader) -> ActorPickRequest {
        let actor_unique_id = PacketSerializer::get_actor_unique_id(stream);
        let add_user_data = stream.get_bool();
        let hotbar_slot = stream.get_u8();

        ActorPickRequest { actor_unique_id, add_user_data, hotbar_slot }
    }
}
