use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::cacheable_nbt::CacheableNBT;
use binary_utils::binary::Stream;
use mojang_nbt::tag::tag::Tag;
use std::any::Any;

#[derive(serde::Serialize, Debug)]
pub struct PositionTrackingDBServerBroadcast {
    pub action: u8,
    pub tracking_id: i32,
    pub nbt: CacheableNBT,
}

impl Packet for PositionTrackingDBServerBroadcast {
    fn id(&self) -> u16 {
        BedrockPacketType::IDPositionTrackingDBServerBroadcast.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_byte(self.action);
        stream.put_var_i32(self.tracking_id);
        stream.put(self.nbt.get_encoded_nbt());

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> PositionTrackingDBServerBroadcast {
        let action = stream.get_byte();
        let tracking_id = stream.get_var_i32();
        let nbt = CacheableNBT::new(Tag::Compound(PacketSerializer::get_nbt_compound_root(
            stream,
        )));

        PositionTrackingDBServerBroadcast { action, tracking_id, nbt }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String { serde_json::to_string(self).unwrap() }
}

impl PositionTrackingDBServerBroadcast {
    pub const ACTION_UPDATE: u8 = 0;
    pub const ACTION_DESTROY: u8 = 1;
    pub const ACTION_NOT_FOUND: u8 = 2;
}
