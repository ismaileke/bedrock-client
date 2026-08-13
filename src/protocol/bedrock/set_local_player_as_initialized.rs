use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct SetLocalPlayerAsInitializedPacket {
    pub actor_runtime_id: u64,
}

impl Packet for SetLocalPlayerAsInitializedPacket {
    fn id(&self) -> u16 {
        BedrockPacketType::IDSetLocalPlayerAsInitialized.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_actor_runtime_id(stream, self.actor_runtime_id);
    }

    fn decode(stream: &mut Reader) -> SetLocalPlayerAsInitializedPacket {
        let actor_runtime_id = PacketSerializer::get_actor_runtime_id(stream);

        SetLocalPlayerAsInitializedPacket { actor_runtime_id }
    }
}
