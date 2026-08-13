use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::cacheable_nbt::CacheableNBT;
use binary_utils::binary::{Reader, Writer};
use mojang_nbt::tag::tag::Tag;

#[derive(serde::Serialize, Debug)]
pub struct JigsawStructureData {
    pub nbt: CacheableNBT,
}

impl Packet for JigsawStructureData {
    fn id(&self) -> u16 {
        BedrockPacketType::IDJigsawStructureData.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put(self.nbt.get_encoded_nbt());
    }

    fn decode(stream: &mut Reader) -> JigsawStructureData {
        let nbt = CacheableNBT::new(Tag::Compound(PacketSerializer::get_nbt_compound_root(
            stream,
        )));

        JigsawStructureData { nbt }
    }
}
