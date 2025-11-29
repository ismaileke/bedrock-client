use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use mojang_nbt::tag::tag::Tag;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::cacheable_nbt::CacheableNBT;

#[derive(serde::Serialize, Debug)]
pub struct JigsawStructureData {
    pub nbt: CacheableNBT
}

pub fn new(nbt: CacheableNBT) -> JigsawStructureData {
    JigsawStructureData { nbt }
}

impl Packet for JigsawStructureData {
    fn id(&self) -> u16 {
        BedrockPacketType::IDJigsawStructureData.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put(self.nbt.get_encoded_nbt());

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> JigsawStructureData {
        let nbt = CacheableNBT::new(Tag::Compound(PacketSerializer::get_nbt_compound_root(stream)));

        JigsawStructureData { nbt }
    }

    fn debug(&self) {
        println!("NBT: {:?}", self.nbt);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
