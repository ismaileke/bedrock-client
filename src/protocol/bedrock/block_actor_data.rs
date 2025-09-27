use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::cacheable_nbt::CacheableNBT;

pub struct BlockActorData {
    pub block_position: Vec<i32>,
    pub nbt: CacheableNBT
}

pub fn new(block_position: Vec<i32>, nbt: CacheableNBT) -> BlockActorData {
    BlockActorData { block_position, nbt }
}

impl Packet for BlockActorData {
    fn id(&self) -> u16 {
        BedrockPacketType::IDBlockActorData.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        PacketSerializer::put_block_pos(&mut stream, self.block_position.clone());
        stream.put(self.nbt.get_encoded_nbt());

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> BlockActorData {
        let mut stream = Stream::new(bytes, 0);

        let block_position = PacketSerializer::get_block_pos(&mut stream);
        let nbt = CacheableNBT::new(Box::new(PacketSerializer::get_nbt_compound_root(&mut stream)));

        BlockActorData { block_position, nbt }
    }

    fn debug(&self) {
        println!("Block Position: {:?}", self.block_position);
        println!("NBT: {:?}", self.nbt);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
