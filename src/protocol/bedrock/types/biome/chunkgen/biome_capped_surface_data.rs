use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

#[derive(serde::Serialize, Debug)]
pub struct BiomeCappedSurfaceData {
    pub floor_blocks: Vec<u32>,
    pub ceiling_blocks: Vec<u32>,
    pub sea_block: Option<u32>,
    pub foundation_block: Option<u32>,
    pub beacon_block: Option<u32>
}

impl BiomeCappedSurfaceData {
    pub fn new(floor_blocks: Vec<u32>, ceiling_blocks: Vec<u32>, sea_block: Option<u32>, foundation_block: Option<u32>, beacon_block: Option<u32>) -> Self {
        BiomeCappedSurfaceData{ floor_blocks, ceiling_blocks, sea_block, foundation_block, beacon_block }
    }

    pub fn read(stream: &mut Stream) -> BiomeCappedSurfaceData {
        let mut floor_blocks = Vec::new();
        let mut ceiling_blocks = Vec::new();

        let mut count = stream.get_var_u32();
        for _ in 0..count {
            floor_blocks.push(stream.get_u32_le());
        }
        count = stream.get_var_u32();
        for _ in 0..count {
            ceiling_blocks.push(stream.get_u32_le());
        }
        let sea_block = PacketSerializer::read_optional(stream, |s| s.get_u32_le());
        let foundation_block = PacketSerializer::read_optional(stream, |s| s.get_u32_le());
        let beacon_block = PacketSerializer::read_optional(stream, |s| s.get_u32_le());

        BiomeCappedSurfaceData{ floor_blocks, ceiling_blocks, sea_block, foundation_block, beacon_block }
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_var_u32(self.floor_blocks.len() as u32);
        for floor_block in &self.floor_blocks {
            stream.put_u32_le(*floor_block);
        }
        stream.put_var_u32(self.ceiling_blocks.len() as u32);
        for ceiling_block in &self.ceiling_blocks {
            stream.put_u32_le(*ceiling_block);
        }
        PacketSerializer::write_optional(stream, &self.sea_block, |s, v| s.put_u32_le(*v));
        PacketSerializer::write_optional(stream, &self.foundation_block, |s, v| s.put_u32_le(*v));
        PacketSerializer::write_optional(stream, &self.beacon_block, |s, v| s.put_u32_le(*v));
    }
}