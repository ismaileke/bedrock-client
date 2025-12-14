use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct CameraAimAssistPresetExclusionDefinition {
    pub blocks: Vec<String>,
    pub entities: Vec<String>,
    pub block_tags: Vec<String>,
}

impl CameraAimAssistPresetExclusionDefinition {
    pub fn new(
        blocks: Vec<String>,
        entities: Vec<String>,
        block_tags: Vec<String>,
    ) -> CameraAimAssistPresetExclusionDefinition {
        CameraAimAssistPresetExclusionDefinition {
            blocks,
            entities,
            block_tags,
        }
    }

    pub fn read(stream: &mut Stream) -> CameraAimAssistPresetExclusionDefinition {
        let mut blocks = Vec::new();
        let mut entities = Vec::new();
        let mut block_tags = Vec::new();

        let mut len = stream.get_var_u32();
        for _ in 0..len {
            blocks.push(PacketSerializer::get_string(stream));
        }

        len = stream.get_var_u32();
        for _ in 0..len {
            entities.push(PacketSerializer::get_string(stream));
        }

        len = stream.get_var_u32();
        for _ in 0..len {
            block_tags.push(PacketSerializer::get_string(stream));
        }

        CameraAimAssistPresetExclusionDefinition {
            blocks,
            entities,
            block_tags,
        }
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_var_u32(self.blocks.len() as u32);
        for block in &self.blocks {
            PacketSerializer::put_string(stream, block.clone());
        }

        stream.put_var_u32(self.entities.len() as u32);
        for entity in &self.entities {
            PacketSerializer::put_string(stream, entity.clone());
        }

        stream.put_var_u32(self.block_tags.len() as u32);
        for block_tag in &self.block_tags {
            PacketSerializer::put_string(stream, block_tag.clone());
        }
    }
}
