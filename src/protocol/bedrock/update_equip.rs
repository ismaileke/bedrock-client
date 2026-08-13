use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::cacheable_nbt::CacheableNBT;
use binary_utils::binary::{Reader, Writer};
use mojang_nbt::tag::tag::Tag;

#[derive(serde::Serialize, Debug)]
pub struct UpdateEquip {
    pub window_id: u8,
    pub window_type: u8,
    pub window_slot_count: i32,
    pub actor_unique_id: i64,
    pub nbt: CacheableNBT,
}

impl Packet for UpdateEquip {
    fn id(&self) -> u16 {
        BedrockPacketType::IDUpdateEquip.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_u8(self.window_id);
        stream.put_u8(self.window_type);
        stream.put_var_i32(self.window_slot_count);
        PacketSerializer::put_actor_unique_id(stream, self.actor_unique_id);
        stream.put(self.nbt.get_encoded_nbt());
    }

    fn decode(stream: &mut Reader) -> UpdateEquip {
        let window_id = stream.get_u8();
        let window_type = stream.get_u8();
        let window_slot_count = stream.get_var_i32();
        let actor_unique_id = PacketSerializer::get_actor_unique_id(stream);
        let nbt = CacheableNBT::new(Tag::Compound(PacketSerializer::get_nbt_compound_root(stream)));

        UpdateEquip {
            window_id,
            window_type,
            window_slot_count,
            actor_unique_id,
            nbt,
        }
    }
}
