use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::types::sub_chunk_position_offset::SubChunkPositionOffset;
use binary_utils::binary::{Reader, Writer};
use crate::protocol::bedrock::types::sub_chunk_position::SubChunkPosition;

#[derive(serde::Serialize, Debug)]
pub struct SubChunkRequest {
    pub dimension: i32,
    pub base_position: SubChunkPosition,
    pub entries: Vec<SubChunkPositionOffset>,
}

impl Packet for SubChunkRequest {
    fn id(&self) -> u16 {
        BedrockPacketType::IDSubChunkRequest.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_var_i32(self.dimension);
        stream.put_var_u32(self.entries.len() as u32);
        for entry in &self.entries {
            entry.write(stream);
        }
        self.base_position.write_fixed_ints(stream);
    }

    fn decode(stream: &mut Reader) -> SubChunkRequest {
        let dimension = stream.get_var_i32();
        let len = stream.get_var_u32() as usize;
        let mut entries = vec![];
        for _ in 0..len {
            entries.push(SubChunkPositionOffset::read(stream));
        }
        let base_position = SubChunkPosition::read_fixed_ints(stream);

        SubChunkRequest { dimension, base_position, entries }
    }
}
