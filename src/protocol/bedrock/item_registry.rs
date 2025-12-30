use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::cacheable_nbt::CacheableNBT;
use crate::protocol::bedrock::types::item_type_entry::ItemTypeEntry;
use binary_utils::binary::Stream;
use mojang_nbt::tag::tag::Tag;
use std::any::Any;

#[derive(serde::Serialize, Debug)]
pub struct ItemRegistry {
    pub entries: Vec<ItemTypeEntry>,
}

impl Packet for ItemRegistry {
    fn id(&self) -> u16 {
        BedrockPacketType::IDItemRegistry.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_var_u32(self.entries.len() as u32);
        for entry in self.entries.iter_mut() {
            PacketSerializer::put_string(&mut stream, entry.string_id.clone());
            stream.put_i16_le(entry.numeric_id);
            stream.put_bool(entry.component_based);
            stream.put_var_i32(entry.version);
            stream.put(entry.component_nbt.get_encoded_nbt());
        }

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> ItemRegistry {
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

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String { serde_json::to_string(self).unwrap() }
}
