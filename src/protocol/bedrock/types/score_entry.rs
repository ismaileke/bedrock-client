#[derive(serde::Serialize, Debug)]
pub struct ScoreEntry {
    pub scoreboard_id: i64,
    pub objective_name: Option<String>,
    pub score: i32,
    pub entity_type: u32,
    pub actor_unique_id: Option<i64>,
    pub custom_name: Option<String>,
}
