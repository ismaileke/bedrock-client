#[derive(serde::Serialize, Debug)]
pub struct StructureSettings {
    pub palette_name: String,
    pub ignore_entities: bool,
    pub ignore_blocks: bool,
    pub allow_non_ticking_chunks: bool,
    pub dimensions: Vec<i32>,
    pub offset: Vec<i32>,
    pub last_touched_by_player_id: i64,
    pub rotation: u8,
    pub mirror: u8,
    pub animation_mode: u8,
    pub animation_seconds: f32,
    pub integrity_value: f32,
    pub integrity_seed: u32,
    pub pivot: Vec<f32>
}
