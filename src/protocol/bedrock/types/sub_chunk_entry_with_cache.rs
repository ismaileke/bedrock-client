use binary_utils::binary::Stream;
use crate::protocol::bedrock::types::sub_chunk_entry_common::SubChunkEntryCommon;

#[derive(Debug)]
pub struct SubChunkEntryWithCache {
    base: SubChunkEntryCommon,
    used_blob_hash: i64
}

impl SubChunkEntryWithCache {
    pub fn new(base: SubChunkEntryCommon, used_blob_hash: i64) -> SubChunkEntryWithCache {
        SubChunkEntryWithCache{ base, used_blob_hash }
    }

    pub fn get_base(&self) -> &SubChunkEntryCommon {
        &self.base
    }

    pub fn get_used_blob_hash(&self) -> i64 {
        self.used_blob_hash
    }

    pub fn read(stream: &mut Stream) -> SubChunkEntryWithCache {
        let base = SubChunkEntryCommon::read(stream, true);
        let used_blob_hash = stream.get_l_long();

        SubChunkEntryWithCache{ base, used_blob_hash }
    }

    pub fn write(&self, stream: &mut Stream) {
        self.base.write(stream, true);
        stream.put_l_long(self.used_blob_hash);
    }
}