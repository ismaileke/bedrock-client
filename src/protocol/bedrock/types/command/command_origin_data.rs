#[derive(serde::Serialize, Debug)]
pub struct CommandOriginData {
    pub origin_type: u32,
    pub uuid: String,
    pub request_id: String,
    pub player_actor_unique_id: i64,
}

impl CommandOriginData {
    pub const ORIGIN_PLAYER: u32 = 0;
    pub const ORIGIN_BLOCK: u32 = 1;
    pub const ORIGIN_MINECART_BLOCK: u32 = 2;
    pub const ORIGIN_DEV_CONSOLE: u32 = 3;
    pub const ORIGIN_TEST: u32 = 4;
    pub const ORIGIN_AUTOMATION_PLAYER: u32 = 5;
    pub const ORIGIN_CLIENT_AUTOMATION: u32 = 6;
    pub const ORIGIN_DEDICATED_SERVER: u32 = 7;
    pub const ORIGIN_ENTITY: u32 = 8;
    pub const ORIGIN_VIRTUAL: u32 = 9;
    pub const ORIGIN_GAME_ARGUMENT: u32 = 10;
    pub const ORIGIN_ENTITY_SERVER: u32 = 11; //???
}
