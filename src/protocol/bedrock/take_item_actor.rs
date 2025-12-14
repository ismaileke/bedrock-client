use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::Stream;
use std::any::Any;

#[derive(serde::Serialize, Debug)]
pub struct TakeItemActor {
    pub item_actor_runtime_id: u64,
    pub taker_actor_runtime_id: u64,
}

pub fn new(item_actor_runtime_id: u64, taker_actor_runtime_id: u64) -> TakeItemActor {
    TakeItemActor {
        item_actor_runtime_id,
        taker_actor_runtime_id,
    }
}

impl Packet for TakeItemActor {
    fn id(&self) -> u16 {
        BedrockPacketType::IDTakeItemActor.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        PacketSerializer::put_actor_runtime_id(&mut stream, self.item_actor_runtime_id);
        PacketSerializer::put_actor_runtime_id(&mut stream, self.taker_actor_runtime_id);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> TakeItemActor {
        let item_actor_runtime_id = PacketSerializer::get_actor_runtime_id(stream);
        let taker_actor_runtime_id = PacketSerializer::get_actor_runtime_id(stream);

        TakeItemActor {
            item_actor_runtime_id,
            taker_actor_runtime_id,
        }
    }

    fn debug(&self) {
        println!("Item Actor Runtime ID: {}", self.item_actor_runtime_id);
        println!("Taker Actor Runtime ID: {}", self.taker_actor_runtime_id);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
