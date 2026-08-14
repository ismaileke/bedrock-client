use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::entity::entity_link::EntityLink;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct SetActorLink {
    pub link: EntityLink,
}

impl Packet for SetActorLink {
    fn id(&self) -> u16 {
        BedrockPacketType::IDSetActorLink.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_entity_link(stream, &self.link);
    }

    fn decode(stream: &mut Reader) -> SetActorLink {
        let link = PacketSerializer::get_entity_link(stream);

        SetActorLink { link }
    }
}
