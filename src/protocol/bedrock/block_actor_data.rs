use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::cacheable_nbt::CacheableNBT;
use binary_utils::binary::{Reader, Writer};
use mojang_nbt::tag::tag::Tag;

#[derive(serde::Serialize, Debug)]
pub struct BlockActorData {
    pub block_position: Vec<i32>,
    pub nbt: CacheableNBT,
}

impl Packet for BlockActorData {
    fn id(&self) -> u16 {
        BedrockPacketType::IDBlockActorData.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_block_pos(stream, &self.block_position);
        stream.put(self.nbt.get_encoded_nbt());
    }

    fn decode(stream: &mut Reader) -> BlockActorData {
        let block_position = PacketSerializer::get_block_pos(stream);
        let nbt = CacheableNBT::new(Tag::Compound(PacketSerializer::get_nbt_compound_root(stream)));

        BlockActorData { block_position, nbt }
    }
}
