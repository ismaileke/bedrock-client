use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

pub struct TakeItemActor {
    pub item_actor_runtime_id: u64,
    pub taker_actor_runtime_id: u64
}

pub fn new(item_actor_runtime_id: u64, taker_actor_runtime_id: u64) -> TakeItemActor {
    TakeItemActor { item_actor_runtime_id, taker_actor_runtime_id }
}

impl Packet for TakeItemActor {
    fn id(&self) -> u16 {
        BedrockPacketType::IDTakeItemActor.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        PacketSerializer::put_actor_runtime_id(&mut stream, self.item_actor_runtime_id);
        PacketSerializer::put_actor_runtime_id(&mut stream, self.taker_actor_runtime_id);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> TakeItemActor {
        let mut stream = Stream::new(bytes, 0);

        let item_actor_runtime_id = PacketSerializer::get_actor_runtime_id(&mut stream);
        let taker_actor_runtime_id = PacketSerializer::get_actor_runtime_id(&mut stream);

        TakeItemActor { item_actor_runtime_id, taker_actor_runtime_id }
    }

    fn debug(&self) {
        println!("Item Actor Runtime ID: {}", self.item_actor_runtime_id);
        println!("Taker Actor Runtime ID: {}", self.taker_actor_runtime_id);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
