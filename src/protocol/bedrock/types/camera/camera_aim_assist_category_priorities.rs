use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::camera::camera_aim_assist_category_priority::CameraAimAssistCategoryPriority;
use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct CameraAimAssistCategoryPriorities {
    pub entities: Vec<CameraAimAssistCategoryPriority>,
    pub blocks: Vec<CameraAimAssistCategoryPriority>,
    pub block_tags: Vec<CameraAimAssistCategoryPriority>,
    pub entity_type_families: Vec<CameraAimAssistCategoryPriority>,
    pub default_entity_priority: Option<i32>,
    pub default_block_priority: Option<i32>,
}

impl CameraAimAssistCategoryPriorities {
    pub fn new(
        entities: Vec<CameraAimAssistCategoryPriority>,
        blocks: Vec<CameraAimAssistCategoryPriority>,
        block_tags: Vec<CameraAimAssistCategoryPriority>,
        entity_type_families: Vec<CameraAimAssistCategoryPriority>,
        default_entity_priority: Option<i32>,
        default_block_priority: Option<i32>,
    ) -> CameraAimAssistCategoryPriorities {
        CameraAimAssistCategoryPriorities {
            entities,
            blocks,
            block_tags,
            entity_type_families,
            default_entity_priority,
            default_block_priority,
        }
    }

    pub fn read(stream: &mut Stream) -> CameraAimAssistCategoryPriorities {
        let mut entities = Vec::new();

        let mut len = stream.get_var_u32();
        for _ in 0..len {
            entities.push(CameraAimAssistCategoryPriority::read(stream));
        }

        let mut blocks = Vec::new();
        len = stream.get_var_u32();
        for _ in 0..len {
            blocks.push(CameraAimAssistCategoryPriority::read(stream));
        }

        let mut block_tags = Vec::new();
        len = stream.get_var_u32();
        for _ in 0..len {
            block_tags.push(CameraAimAssistCategoryPriority::read(stream));
        }

        let mut entity_type_families = Vec::new();
        len = stream.get_var_u32();
        for _ in 0..len {
            entity_type_families.push(CameraAimAssistCategoryPriority::read(stream));
        }
        let default_entity_priority = PacketSerializer::read_optional(stream, |s| s.get_i32_le());
        let default_block_priority = PacketSerializer::read_optional(stream, |s| s.get_i32_le());

        CameraAimAssistCategoryPriorities {
            entities,
            blocks,
            block_tags,
            entity_type_families,
            default_entity_priority,
            default_block_priority,
        }
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
        stream.put_var_u32(self.block_tags.len() as u32);
        for block_tag in &self.block_tags {
            block_tag.write(stream);
        }
        stream.put_var_u32(self.entity_type_families.len() as u32);
        for family in &self.entity_type_families {
            family.write(stream);
        }
        PacketSerializer::write_optional(stream, &self.default_entity_priority, |s, v| s.put_i32_le(*v));
        PacketSerializer::write_optional(stream, &self.default_block_priority, |s, v| s.put_i32_le(*v));
    }
}
