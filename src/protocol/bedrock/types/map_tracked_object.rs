#[derive(serde::Serialize, Debug)]
pub struct MapTrackedObject {
    pub object_type: i32,
    pub block_position: Option<Vec<i32>>, // TYPE_BLOCK
    pub actor_unique_id: Option<i64> // TYPE_ENTITY
}

impl MapTrackedObject {
    pub const TYPE_ENTITY: i32 = 0;
    pub const TYPE_BLOCK_ENTITY: i32 = 1;
    pub const TYPE_OTHER: i32 = 2;
}