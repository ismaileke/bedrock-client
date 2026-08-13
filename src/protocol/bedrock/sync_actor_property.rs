use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::cacheable_nbt::CacheableNBT;
use binary_utils::binary::{Reader, Writer};
use mojang_nbt::tag::tag::Tag;

#[derive(serde::Serialize, Debug)]
pub struct SyncActorProperty {
    pub nbt: CacheableNBT,
}

impl Packet for SyncActorProperty {
    fn id(&self) -> u16 {
        BedrockPacketType::IDSyncActorProperty.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put(self.nbt.get_encoded_nbt());
    }

    fn decode(stream: &mut Reader) -> SyncActorProperty {
        let nbt = CacheableNBT::new(Tag::Compound(PacketSerializer::get_nbt_compound_root(
            stream,
        )));

        SyncActorProperty { nbt }
    }
}
