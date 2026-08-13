use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::cacheable_nbt::CacheableNBT;
use binary_utils::binary::{Reader, Writer};
use mojang_nbt::tag::tag::Tag;

#[derive(serde::Serialize, Debug)]
pub struct PositionTrackingDBServerBroadcast {
    pub action: u8,
    pub tracking_id: i32,
    pub nbt: CacheableNBT,
}

impl Packet for PositionTrackingDBServerBroadcast {
    fn id(&self) -> u16 {
        BedrockPacketType::IDPositionTrackingDBServerBroadcast.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_u8(self.action);
        stream.put_var_i32(self.tracking_id);
        stream.put(self.nbt.get_encoded_nbt());
    }

    fn decode(stream: &mut Reader) -> PositionTrackingDBServerBroadcast {
        let action = stream.get_u8();
        let tracking_id = stream.get_var_i32();
        let nbt = CacheableNBT::new(Tag::Compound(PacketSerializer::get_nbt_compound_root(
            stream,
        )));

        PositionTrackingDBServerBroadcast { action, tracking_id, nbt }
    }
}

impl PositionTrackingDBServerBroadcast {
    pub const ACTION_UPDATE: u8 = 0;
    pub const ACTION_DESTROY: u8 = 1;
    pub const ACTION_NOT_FOUND: u8 = 2;
}
