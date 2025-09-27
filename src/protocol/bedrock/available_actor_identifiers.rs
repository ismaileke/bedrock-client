use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::cacheable_nbt::CacheableNBT;

pub struct AvailableActorIdentifiers {
    pub identifiers: CacheableNBT
}

pub fn new(identifiers: CacheableNBT) -> AvailableActorIdentifiers {
    AvailableActorIdentifiers { identifiers }
}

impl Packet for AvailableActorIdentifiers {
    fn id(&self) -> u16 {
        BedrockPacketType::IDAvailableActorIdentifiers.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        stream.put(self.identifiers.get_encoded_nbt());

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> AvailableActorIdentifiers {
        let mut stream = Stream::new(bytes, 0);

        let identifiers = CacheableNBT::new(Box::new(PacketSerializer::get_nbt_compound_root(&mut stream)));

        AvailableActorIdentifiers { identifiers }
    }

    fn debug(&self) {
        println!("Identifiers(NBT): {:?}", self.identifiers);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
