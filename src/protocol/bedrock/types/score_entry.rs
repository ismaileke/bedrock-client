#[derive(Debug)]
pub struct ScoreEntry {
    pub scoreboard_id: i64,
    pub objective_name: String,
    pub score: i32,
    pub entity_type: u8,
    pub actor_unique_id: Option<i64>,
    pub custom_name: Option<String>
}

impl ScoreEntry {
    pub const TYPE_PLAYER: u8 = 1;
    pub const TYPE_ENTITY: u8 = 2;
    pub const TYPE_FAKE_PLAYER: u8 = 3;
}
