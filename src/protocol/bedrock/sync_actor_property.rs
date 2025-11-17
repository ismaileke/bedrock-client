use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::cacheable_nbt::CacheableNBT;

pub struct SyncActorProperty {
    pub nbt: CacheableNBT
}

pub fn new(nbt: CacheableNBT) -> SyncActorProperty {
    SyncActorProperty { nbt }
}

impl Packet for SyncActorProperty {
    fn id(&self) -> u16 {
        BedrockPacketType::IDSyncActorProperty.get_byte()
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

    fn decode(bytes: Vec<u8>) -> SyncActorProperty {
        let mut stream = Stream::new(bytes, 0);

        let nbt = CacheableNBT::new(Box::new(PacketSerializer::get_nbt_compound_root(&mut stream)));

        SyncActorProperty { nbt }
    }

    fn debug(&self) {
        println!("NBT: {:?}", self.nbt);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
