use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::cacheable_nbt::CacheableNBT;
use crate::protocol::bedrock::types::item_type_entry::ItemTypeEntry;

pub struct ItemRegistry {
    pub entries: Vec<ItemTypeEntry>
}

pub fn new(entries: Vec<ItemTypeEntry>) -> ItemRegistry {
    ItemRegistry { entries }
}

impl Packet for ItemRegistry {
    fn id(&self) -> u16 {
        BedrockPacketType::IDItemRegistry.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        stream.put_unsigned_var_int(self.entries.len() as u32);
        for entry in self.entries.iter_mut() {
            PacketSerializer::put_string(&mut stream, entry.string_id.clone());
            stream.put_l_short(entry.numeric_id);
            stream.put_bool(entry.component_based);
            stream.put_var_int(entry.version);
            stream.put(entry.component_nbt.get_encoded_nbt());
        }

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> ItemRegistry {
        let mut stream = Stream::new(bytes, 0);

        let entries_len = stream.get_unsigned_var_int() as usize;
        let mut entries = Vec::new();
        for _ in 0..entries_len {
            let string_id = PacketSerializer::get_string(&mut stream);
            let numeric_id = stream.get_l_short();
            let component_based = stream.get_bool();
            let version = stream.get_var_int();
            let component_nbt = PacketSerializer::get_nbt_compound_root(&mut stream);
            entries.push(ItemTypeEntry { string_id, numeric_id, component_based, version, component_nbt: CacheableNBT::new(Box::new(component_nbt)) });
        }

        ItemRegistry { entries }
    }

    fn debug(&self) {
        println!("Entries: {:?}", self.entries);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
