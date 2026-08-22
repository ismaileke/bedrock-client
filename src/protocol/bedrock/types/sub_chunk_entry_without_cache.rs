use crate::protocol::bedrock::types::sub_chunk_entry_common::SubChunkEntryCommon;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct SubChunkEntryWithoutCache {
    base: SubChunkEntryCommon,
}

impl SubChunkEntryWithoutCache {
    pub fn new(base: SubChunkEntryCommon) -> SubChunkEntryWithoutCache {
        SubChunkEntryWithoutCache { base }
    }

    pub fn get_base(&self) -> &SubChunkEntryCommon {
        &self.base
    }

    pub fn read(stream: &mut Reader) -> SubChunkEntryWithoutCache {
        let base = SubChunkEntryCommon::read(stream);
        if stream.get_bool() {
            let _ = stream.get_u64_le(); //blob hash, useless without a cache
        }

        SubChunkEntryWithoutCache { base }
    }

    pub fn write(&self, stream: &mut Writer) {
        self.base.write(stream);
        stream.put_bool(false);
    }
}
