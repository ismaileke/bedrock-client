use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::camera::camera_aim_assist_category_block_priority::CameraAimAssistCategoryBlockPriority;
use crate::protocol::bedrock::types::camera::camera_aim_assist_category_entity_priority::CameraAimAssistCategoryEntityPriority;

#[derive(Debug)]
pub struct CameraAimAssistCategoryPriorities {
    pub entities: Vec<CameraAimAssistCategoryEntityPriority>,
    pub blocks: Vec<CameraAimAssistCategoryBlockPriority>,
    pub default_entity_priority: Option<i32>,
    pub default_block_priority: Option<i32>
}

impl CameraAimAssistCategoryPriorities {
    pub fn new(
        entities: Vec<CameraAimAssistCategoryEntityPriority>,
        blocks: Vec<CameraAimAssistCategoryBlockPriority>,
        default_entity_priority: Option<i32>,
        default_block_priority: Option<i32>
    ) -> CameraAimAssistCategoryPriorities {
        CameraAimAssistCategoryPriorities{ entities, blocks, default_entity_priority, default_block_priority }
    }

    pub fn read(stream: &mut Stream) -> CameraAimAssistCategoryPriorities {
        let mut entities = Vec::new();
        let mut len = stream.get_var_u32();
        for _ in 0..len {
            entities.push(CameraAimAssistCategoryEntityPriority::read(stream));
        }
        let mut blocks = Vec::new();
        len = stream.get_var_u32();
        for _ in 0..len {
            blocks.push(CameraAimAssistCategoryBlockPriority::read(stream));
        }
        let default_entity_priority = PacketSerializer::read_optional(stream, |s| s.get_i32_le());
        let default_block_priority = PacketSerializer::read_optional(stream, |s| s.get_i32_le());

        CameraAimAssistCategoryPriorities{ entities, blocks, default_entity_priority, default_block_priority }
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_var_u32(self.entities.len() as u32);
        for entity in &self.entities {
            entity.write(stream);
        }
        stream.put_var_u32(self.blocks.len() as u32);
        for block in &self.blocks {
            block.write(stream);
        }
        PacketSerializer::write_optional(stream, &self.default_entity_priority, |s, v| s.put_i32_le(*v));
        PacketSerializer::write_optional(stream, &self.default_block_priority, |s, v| s.put_i32_le(*v));
    }
}