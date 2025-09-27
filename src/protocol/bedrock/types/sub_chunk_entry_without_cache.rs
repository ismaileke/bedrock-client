use binary_utils::binary::Stream;
use crate::protocol::bedrock::types::sub_chunk_entry_common::SubChunkEntryCommon;

#[derive(Debug)]
pub struct SubChunkEntryWithoutCache {
    base: SubChunkEntryCommon
}

impl SubChunkEntryWithoutCache {
    pub fn new(base: SubChunkEntryCommon) -> SubChunkEntryWithoutCache {
        SubChunkEntryWithoutCache{ base }
    }

    pub fn get_base(&self) -> &SubChunkEntryCommon {
        &self.base
    }

    pub fn read(stream: &mut Stream) -> SubChunkEntryWithoutCache {
        let base = SubChunkEntryCommon::read(stream, false);

        SubChunkEntryWithoutCache{ base }
    }

    pub fn write(&self, stream: &mut Stream) {
        self.base.write(stream, false);
    }
}