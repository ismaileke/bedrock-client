use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::cacheable_nbt::CacheableNBT;
use binary_utils::binary::{Reader, Writer};
use mojang_nbt::tag::tag::Tag;

#[derive(serde::Serialize, Debug)]
pub struct AvailableActorIdentifiers {
    pub identifiers: CacheableNBT,
}

impl Packet for AvailableActorIdentifiers {
    fn id(&self) -> u16 {
        BedrockPacketType::IDAvailableActorIdentifiers.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put(self.identifiers.get_encoded_nbt());
    }

    fn decode(stream: &mut Reader) -> AvailableActorIdentifiers {
        let identifiers = CacheableNBT::new(Tag::Compound(PacketSerializer::get_nbt_compound_root(stream)));

        AvailableActorIdentifiers { identifiers }
    }
}
