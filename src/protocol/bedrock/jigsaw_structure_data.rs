use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::cacheable_nbt::CacheableNBT;

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
        stream.put_unsigned_var_int(self.id() as u32);

        stream.put(self.nbt.get_encoded_nbt());

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> JigsawStructureData {
        let mut stream = Stream::new(bytes, 0);

        let nbt = CacheableNBT::new(Box::new(PacketSerializer::get_nbt_compound_root(&mut stream)));

        JigsawStructureData { nbt }
    }

    fn debug(&self) {
        println!("NBT: {:?}", self.nbt);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
