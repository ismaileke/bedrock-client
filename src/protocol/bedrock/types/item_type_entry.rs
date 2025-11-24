use crate::protocol::bedrock::types::cacheable_nbt::CacheableNBT;

#[derive(serde::Serialize, Debug)]
pub struct ItemTypeEntry {
    pub string_id: String,
    pub numeric_id: i16,
    pub component_based: bool,
    pub version: i32,
    pub component_nbt: CacheableNBT
}
