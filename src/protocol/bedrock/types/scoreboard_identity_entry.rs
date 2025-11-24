#[derive(serde::Serialize, Debug)]
pub struct ScoreboardIdentityEntry {
    pub scoreboard_id: i64,
    pub actor_unique_id: Option<i64>
}
