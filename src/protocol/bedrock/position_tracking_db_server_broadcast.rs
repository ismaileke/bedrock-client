use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::cacheable_nbt::CacheableNBT;

pub struct PositionTrackingDBServerBroadcast {
    pub action: u8,
    pub tracking_id: i32,
    pub nbt: CacheableNBT
}

pub fn new(action: u8, tracking_id: i32, nbt: CacheableNBT) -> PositionTrackingDBServerBroadcast {
    PositionTrackingDBServerBroadcast { action, tracking_id, nbt }
}

impl Packet for PositionTrackingDBServerBroadcast {
    fn id(&self) -> u16 {
        BedrockPacketType::IDPositionTrackingDBServerBroadcast.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        stream.put_byte(self.action);
        stream.put_var_int(self.tracking_id);
        stream.put(self.nbt.get_encoded_nbt());

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> PositionTrackingDBServerBroadcast {
        let mut stream = Stream::new(bytes, 0);

        let action = stream.get_byte();
        let tracking_id = stream.get_var_int();
        let nbt = CacheableNBT::new(Box::new(PacketSerializer::get_nbt_compound_root(&mut stream)));

        PositionTrackingDBServerBroadcast { action, tracking_id, nbt }
    }

    fn debug(&self) {
        println!("Action: {}", self.action);
        println!("Tracking ID: {}", self.tracking_id);
        println!("NBT: {:?}", self.nbt);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
