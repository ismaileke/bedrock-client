use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::update_sub_chunk_blocks_entry::UpdateSubChunkBlocksEntry;
use binary_utils::binary::Stream;
use std::any::Any;

#[derive(serde::Serialize, Debug)]
pub struct UpdateSubChunkBlocks {
    pub base_block_position: Vec<i32>,
    pub layer_0_updates: Vec<UpdateSubChunkBlocksEntry>,
    pub layer_1_updates: Vec<UpdateSubChunkBlocksEntry>,
}

pub fn new(
    base_block_position: Vec<i32>,
    layer_0_updates: Vec<UpdateSubChunkBlocksEntry>,
    layer_1_updates: Vec<UpdateSubChunkBlocksEntry>,
) -> UpdateSubChunkBlocks {
    UpdateSubChunkBlocks {
        base_block_position,
        layer_0_updates,
        layer_1_updates,
    }
}

impl Packet for UpdateSubChunkBlocks {
    fn id(&self) -> u16 {
        BedrockPacketType::IDUpdateSubChunkBlocks.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        PacketSerializer::put_block_pos(&mut stream, self.base_block_position.clone());
        stream.put_var_u32(self.layer_0_updates.len() as u32);
        for update in self.layer_0_updates.iter() {
            update.write(&mut stream);
        }
        stream.put_var_u32(self.layer_1_updates.len() as u32);
        for update in self.layer_1_updates.iter() {
            update.write(&mut stream);
        }

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> UpdateSubChunkBlocks {
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

        UpdateSubChunkBlocks {
            base_block_position,
            layer_0_updates,
            layer_1_updates,
        }
    }

    fn debug(&self) {
        println!("Base Block Position: {:?}", self.base_block_position);
        println!("Layer 0 Updates: {:?}", self.layer_0_updates);
        println!("Layer 1 Updates: {:?}", self.layer_1_updates);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
