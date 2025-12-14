use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::types::sub_chunk_entry_with_cache::SubChunkEntryWithCache;
use crate::protocol::bedrock::types::sub_chunk_entry_with_cache_list::SubChunkEntryWithCacheList;
use crate::protocol::bedrock::types::sub_chunk_entry_without_cache::SubChunkEntryWithoutCache;
use crate::protocol::bedrock::types::sub_chunk_entry_without_cache_list::SubChunkEntryWithoutCacheList;
use binary_utils::binary::Stream;
use std::any::Any;

#[derive(serde::Serialize, Debug)]
pub struct SubChunk {
    pub dimension: i32,
    pub base_sub_chunk_position: Vec<i32>,
    pub entries: SubChunkEntries,
}

#[derive(serde::Serialize, Debug)]
pub enum SubChunkEntries {
    ListWithCache(SubChunkEntryWithCacheList),
    ListWithoutCache(SubChunkEntryWithoutCacheList),
}

pub fn new(
    dimension: i32,
    base_sub_chunk_position: Vec<i32>,
    entries: SubChunkEntries,
) -> SubChunk {
    SubChunk {
        dimension,
        base_sub_chunk_position,
        entries,
    }
}

impl Packet for SubChunk {
    fn id(&self) -> u16 {
        BedrockPacketType::IDSubChunk.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        let cache_enabled = matches!(self.entries, SubChunkEntries::ListWithCache(_));
        stream.put_bool(cache_enabled);
        stream.put_var_i32(self.dimension);
        for &coord in &self.base_sub_chunk_position {
            stream.put_var_i32(coord);
        }
        match &self.entries {
            SubChunkEntries::ListWithCache(list) => {
                stream.put_u32_le(list.get_entries().len() as u32);
                for entry in list.get_entries() {
                    entry.write(&mut stream);
                }
            }
            SubChunkEntries::ListWithoutCache(list) => {
                stream.put_u32_le(list.get_entries().len() as u32);
                for entry in list.get_entries() {
                    entry.write(&mut stream);
                }
            }
        }; // check later
           /*stream.put_l_int(self.entries.len() as u32);
           for entry in self.entries {
               entry.write(&mut stream);
           }*/

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> SubChunk {
        let cache_enabled = stream.get_bool();
        let dimension = stream.get_var_i32();
        let x = stream.get_var_i32();
        let y = stream.get_var_i32();
        let z = stream.get_var_i32();
        let base_sub_chunk_position = vec![x, y, z];

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

        SubChunk {
            dimension,
            base_sub_chunk_position,
            entries,
        }
    }

    fn debug(&self) {
        println!("Dimension: {:?}", self.dimension);
        println!(
            "Base sub chunk position: {:?}",
            self.base_sub_chunk_position
        );
        println!("Entries: {:?}", self.entries);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
