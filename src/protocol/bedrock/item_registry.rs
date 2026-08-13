use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::cacheable_nbt::CacheableNBT;
use crate::protocol::bedrock::types::item_type_entry::ItemTypeEntry;
use binary_utils::binary::{Reader, Writer};
use mojang_nbt::tag::tag::Tag;

#[derive(serde::Serialize, Debug)]
pub struct ItemRegistry {
    pub entries: Vec<ItemTypeEntry>,
}

impl Packet for ItemRegistry {
    fn id(&self) -> u16 {
        BedrockPacketType::IDItemRegistry.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_var_u32(self.entries.len() as u32);
        for entry in self.entries.iter_mut() {
            PacketSerializer::put_string(stream, entry.string_id.clone());
            stream.put_i16_le(entry.numeric_id);
            stream.put_bool(entry.component_based);
            stream.put_var_i32(entry.version);
            stream.put(entry.component_nbt.get_encoded_nbt());
        }
    }

    fn decode(stream: &mut Reader) -> ItemRegistry {
        let entries_len = stream.get_var_u32() as usize;
        let mut entries = Vec::new();
        for _ in 0..entries_len {
            let string_id = PacketSerializer::get_string(stream);
            let numeric_id = stream.get_i16_le();
            let component_based = stream.get_bool();
            let version = stream.get_var_i32();
            let component_nbt = PacketSerializer::get_nbt_compound_root(stream);
            entries.push(ItemTypeEntry {
                string_id,
                numeric_id,
                component_based,
                version,
                component_nbt: CacheableNBT::new(Tag::Compound(component_nbt)),
            });
        }

        ItemRegistry { entries }
    }
}
