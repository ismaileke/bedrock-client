use crate::protocol::bedrock::types::sub_chunk_entry_without_cache::SubChunkEntryWithoutCache;

#[derive(Debug)]
pub struct SubChunkEntryWithoutCacheList {
    entries: Vec<SubChunkEntryWithoutCache>
}

impl SubChunkEntryWithoutCacheList {
    pub fn new(entries: Vec<SubChunkEntryWithoutCache>) -> SubChunkEntryWithoutCacheList {
        SubChunkEntryWithoutCacheList{ entries }
    }
    
    pub fn get_entries(&self) -> &Vec<SubChunkEntryWithoutCache> {
        self.entries.as_ref()
    }
}