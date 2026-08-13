use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::types::sub_chunk_entry_with_cache::SubChunkEntryWithCache;
use crate::protocol::bedrock::types::sub_chunk_entry_with_cache_list::SubChunkEntryWithCacheList;
use crate::protocol::bedrock::types::sub_chunk_entry_without_cache::SubChunkEntryWithoutCache;
use crate::protocol::bedrock::types::sub_chunk_entry_without_cache_list::SubChunkEntryWithoutCacheList;
use crate::protocol::bedrock::types::sub_chunk_position::SubChunkPosition;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct SubChunk {
    pub dimension: i32,
    pub base_sub_chunk_position: SubChunkPosition,
    pub entries: SubChunkEntries,
}

#[derive(serde::Serialize, Debug)]
pub enum SubChunkEntries {
    ListWithCache(SubChunkEntryWithCacheList),
    ListWithoutCache(SubChunkEntryWithoutCacheList),
}

impl Packet for SubChunk {
    fn id(&self) -> u16 {
        BedrockPacketType::IDSubChunk.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        let cache_enabled = matches!(self.entries, SubChunkEntries::ListWithCache(_));
        stream.put_bool(cache_enabled);
        stream.put_var_i32(self.dimension);
        self.base_sub_chunk_position.write_var_ints(stream);
        match &self.entries {
            SubChunkEntries::ListWithCache(list) => {
                stream.put_u32_le(list.get_entries().len() as u32);
                for entry in list.get_entries() {
                    entry.write(stream);
                }
            }
            SubChunkEntries::ListWithoutCache(list) => {
                stream.put_u32_le(list.get_entries().len() as u32);
                for entry in list.get_entries() {
                    entry.write(stream);
                }
            }
        }; // check later
           /*stream.put_u32_le(self.entries.len() as u32);
           for entry in self.entries {
               entry.write(stream);
           }*/
    }

    fn decode(stream: &mut Reader) -> SubChunk {
        let cache_enabled = stream.get_bool();
        let dimension = stream.get_var_i32();
        let base_sub_chunk_position = SubChunkPosition::read_var_ints(stream);
        let count = stream.get_u32_le();
        let entries = if cache_enabled {
            let mut sub_entries = Vec::new();
            for _ in 0..count {
                sub_entries.push(SubChunkEntryWithCache::read(stream));
            }
            SubChunkEntries::ListWithCache(SubChunkEntryWithCacheList::new(sub_entries))
        } else {
            let mut sub_entries = Vec::new();
            for _ in 0..count {
                sub_entries.push(SubChunkEntryWithoutCache::read(stream));
            }
            SubChunkEntries::ListWithoutCache(SubChunkEntryWithoutCacheList::new(sub_entries))
        };

        SubChunk { dimension, base_sub_chunk_position, entries }
    }
}
