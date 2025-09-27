use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::camera::camera_aim_assist_category_block_priority::CameraAimAssistCategoryBlockPriority;
use crate::protocol::bedrock::types::camera::camera_aim_assist_category_entity_priority::CameraAimAssistCategoryEntityPriority;

#[derive(Debug)]
pub struct CameraAimAssistCategoryPriorities {
    entities: Vec<CameraAimAssistCategoryEntityPriority>,
    blocks: Vec<CameraAimAssistCategoryBlockPriority>,
    default_entity_priority: Option<u32>,
    default_block_priority: Option<u32>
}

impl CameraAimAssistCategoryPriorities {
    pub fn new(
        entities: Vec<CameraAimAssistCategoryEntityPriority>,
        blocks: Vec<CameraAimAssistCategoryBlockPriority>,
        default_entity_priority: Option<u32>,
        default_block_priority: Option<u32>
    ) -> CameraAimAssistCategoryPriorities {
        CameraAimAssistCategoryPriorities{ entities, blocks, default_entity_priority, default_block_priority }
    }

    pub fn read(stream: &mut Stream) -> CameraAimAssistCategoryPriorities {
        let mut entities = Vec::new();
        let mut len = stream.get_unsigned_var_int();
        for _ in 0..len {
            entities.push(CameraAimAssistCategoryEntityPriority::read(stream));
        }
        let mut blocks = Vec::new();
        len = stream.get_unsigned_var_int();
        for _ in 0..len {
            blocks.push(CameraAimAssistCategoryBlockPriority::read(stream));
        }
        let default_entity_priority = PacketSerializer::read_optional(stream, |s| s.get_l_int());
        let default_block_priority = PacketSerializer::read_optional(stream, |s| s.get_l_int());

        CameraAimAssistCategoryPriorities{ entities, blocks, default_entity_priority, default_block_priority }
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_unsigned_var_int(self.entities.len() as u32);
        for entity in &self.entities {
            entity.write(stream);
        }
        stream.put_unsigned_var_int(self.blocks.len() as u32);
        for block in &self.blocks {
            block.write(stream);
        }
        PacketSerializer::write_optional(stream, &self.default_entity_priority, |s, v| s.put_l_int(*v));
        PacketSerializer::write_optional(stream, &self.default_block_priority, |s, v| s.put_l_int(*v));
    }
}