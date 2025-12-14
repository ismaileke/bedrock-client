use crate::protocol::bedrock::types::sub_chunk_entry_common::SubChunkEntryCommon;
use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct SubChunkEntryWithCache {
    base: SubChunkEntryCommon,
    used_blob_hash: u64,
}

impl SubChunkEntryWithCache {
    pub fn new(base: SubChunkEntryCommon, used_blob_hash: u64) -> SubChunkEntryWithCache {
        SubChunkEntryWithCache {
            base,
            used_blob_hash,
        }
    }

    pub fn get_base(&self) -> &SubChunkEntryCommon {
        &self.base
    }

    pub fn get_used_blob_hash(&self) -> u64 {
        self.used_blob_hash
    }

    pub fn read(stream: &mut Stream) -> SubChunkEntryWithCache {
        let base = SubChunkEntryCommon::read(stream, true);
        let used_blob_hash = stream.get_u64_le();

        SubChunkEntryWithCache {
            base,
            used_blob_hash,
        }
    }

    pub fn write(&self, stream: &mut Stream) {
        self.base.write(stream, true);
        stream.put_u64_le(self.used_blob_hash);
    }
}
