use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::update_block::UpdateBlock;

#[derive(serde::Serialize, Debug)]
pub struct UpdateSubChunkBlocksEntry {
    block_position: Vec<i32>,
    block_runtime_id: u32,
    flags: u32,
    //These two fields are useless 99.9% of the time; they are here to allow this packet to provide UpdateBlockSyncedPacket functionality.
    synced_update_actor_unique_id: u64,
    synced_update_type: u32
}

impl UpdateSubChunkBlocksEntry {
    pub fn new(block_position: Vec<i32>, block_runtime_id: u32, flags: u32, synced_update_actor_unique_id: u64, synced_update_type: u32) -> UpdateSubChunkBlocksEntry {
        UpdateSubChunkBlocksEntry{ block_position, block_runtime_id, flags, synced_update_actor_unique_id, synced_update_type }
    }

    pub fn simple(block_position: Vec<i32>, block_runtime_id: u32) -> UpdateSubChunkBlocksEntry {
        UpdateSubChunkBlocksEntry::new(block_position, block_runtime_id, UpdateBlock::FLAG_NETWORK, 0, 0)
    }

    pub fn read(stream: &mut Stream) -> UpdateSubChunkBlocksEntry {
        let block_position = PacketSerializer::get_block_pos(stream);
        let block_runtime_id = stream.get_var_u32();
        let flags = stream.get_var_u32();
        let synced_update_actor_unique_id = stream.get_var_u64();
        let synced_update_type = stream.get_var_u32();

        UpdateSubChunkBlocksEntry{ block_position, block_runtime_id, flags, synced_update_actor_unique_id, synced_update_type }
    }

    pub fn write(&self, stream: &mut Stream) {
        PacketSerializer::put_block_pos(stream, self.block_position.clone());
        stream.put_var_u32(self.block_runtime_id);
        stream.put_var_u32(self.flags);
        stream.put_var_u64(self.synced_update_actor_unique_id);
        stream.put_var_u32(self.synced_update_type);
    }
}