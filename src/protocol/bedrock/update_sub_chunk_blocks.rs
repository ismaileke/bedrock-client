use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::update_sub_chunk_blocks_entry::UpdateSubChunkBlocksEntry;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct UpdateSubChunkBlocks {
    pub base_block_position: Vec<i32>,
    pub layer_0_updates: Vec<UpdateSubChunkBlocksEntry>,
    pub layer_1_updates: Vec<UpdateSubChunkBlocksEntry>,
}

impl Packet for UpdateSubChunkBlocks {
    fn id(&self) -> u16 {
        BedrockPacketType::IDUpdateSubChunkBlocks.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_block_pos(stream, &self.base_block_position);
        stream.put_var_u32(self.layer_0_updates.len() as u32);
        for update in self.layer_0_updates.iter() {
            update.write(stream);
        }
        stream.put_var_u32(self.layer_1_updates.len() as u32);
        for update in self.layer_1_updates.iter() {
            update.write(stream);
        }
    }

    fn decode(stream: &mut Reader) -> UpdateSubChunkBlocks {
        let base_block_position = PacketSerializer::get_block_pos(stream);
        let layer_0_updates_count = stream.get_var_u32() as usize;
        let mut layer_0_updates = Vec::new();
        for _ in 0..layer_0_updates_count {
            layer_0_updates.push(UpdateSubChunkBlocksEntry::read(stream));
        }
        let layer_1_updates_count = stream.get_var_u32() as usize;
        let mut layer_1_updates = Vec::new();
        for _ in 0..layer_1_updates_count {
            layer_1_updates.push(UpdateSubChunkBlocksEntry::read(stream));
        }

        UpdateSubChunkBlocks { base_block_position, layer_0_updates, layer_1_updates }
    }
}
