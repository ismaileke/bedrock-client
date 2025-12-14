#[derive(serde::Serialize, Debug)]
pub struct CommandOriginData {
    pub origin_type: String,
    pub uuid: String,
    pub request_id: String,
    pub player_actor_unique_id: i64,
}

impl CommandOriginData {
    pub const ORIGIN_PLAYER: &'static str = "player";
    pub const ORIGIN_BLOCK: &'static str = "commandblock";
    pub const ORIGIN_MINECART_BLOCK: &'static str = "minecartcommandblock";
    pub const ORIGIN_DEV_CONSOLE: &'static str = "devconsole";
    pub const ORIGIN_TEST: &'static str = "test";
    pub const ORIGIN_AUTOMATION_PLAYER: &'static str = "automationplayer";
    pub const ORIGIN_CLIENT_AUTOMATION: &'static str = "clientautomation";
    pub const ORIGIN_DEDICATED_SERVER: &'static str = "dedicatedserver";
    pub const ORIGIN_ENTITY: &'static str = "entity";
    pub const ORIGIN_VIRTUAL: &'static str = "virtual";
    pub const ORIGIN_GAME_ARGUMENT: &'static str = "gameargument";
    pub const ORIGIN_ENTITY_SERVER: &'static str = "entityserver";
    pub const ORIGIN_PRECOMPILED: &'static str = "precompiled";
    pub const ORIGIN_GAME_DIRECTOR_ENTITY_SERVER: &'static str = "gamedirectorentityserver";
    pub const ORIGIN_SCRIPTING: &'static str = "scripting";
    pub const ORIGIN_EXECUTE_CONTEXT: &'static str = "executecontext";
}
