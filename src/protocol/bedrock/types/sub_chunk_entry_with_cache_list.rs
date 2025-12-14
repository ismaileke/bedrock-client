use crate::protocol::bedrock::types::sub_chunk_entry_with_cache::SubChunkEntryWithCache;

#[derive(serde::Serialize, Debug)]
pub struct SubChunkEntryWithCacheList {
    entries: Vec<SubChunkEntryWithCache>,
}

impl SubChunkEntryWithCacheList {
    pub fn new(entries: Vec<SubChunkEntryWithCache>) -> SubChunkEntryWithCacheList {
        SubChunkEntryWithCacheList { entries }
    }

    pub fn get_entries(&self) -> &Vec<SubChunkEntryWithCache> {
        self.entries.as_ref()
    }
}
