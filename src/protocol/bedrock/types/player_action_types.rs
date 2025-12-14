pub struct PlayerActionTypes {}

impl PlayerActionTypes {
    pub const START_BREAK: i32 = 0;
    pub const ABORT_BREAK: i32 = 1;
    pub const STOP_BREAK: i32 = 2;
    pub const GET_UPDATED_BLOCK: i32 = 3;
    pub const DROP_ITEM: i32 = 4;
    pub const START_SLEEPING: i32 = 5;
    pub const STOP_SLEEPING: i32 = 6;
    pub const RESPAWN: i32 = 7;
    pub const JUMP: i32 = 8;
    pub const START_SPRINT: i32 = 9;
    pub const STOP_SPRINT: i32 = 10;
    pub const START_SNEAK: i32 = 11;
    pub const STOP_SNEAK: i32 = 12;
    pub const CREATIVE_PLAYER_DESTROY_BLOCK: i32 = 13;
    pub const DIMENSION_CHANGE_ACK: i32 = 14; //sent when spawning in a different dimension to tell the server we spawned
    pub const START_GLIDE: i32 = 15;
    pub const STOP_GLIDE: i32 = 16;
    pub const BUILD_DENIED: i32 = 17;
    pub const CRACK_BLOCK: i32 = 18;
    pub const CHANGE_SKIN: i32 = 19;
    pub const SET_ENCHANTMENT_SEED: i32 = 20; //no longer used
    pub const START_SWIMMING: i32 = 21;
    pub const STOP_SWIMMING: i32 = 22;
    pub const START_SPIN_ATTACK: i32 = 23;
    pub const STOP_SPIN_ATTACK: i32 = 24;
    pub const INTERACT_BLOCK: i32 = 25;
    pub const PREDICT_DESTROY_BLOCK: i32 = 26;
    pub const CONTINUE_DESTROY_BLOCK: i32 = 27;
    pub const START_ITEM_USE_ON: i32 = 28;
    pub const STOP_ITEM_USE_ON: i32 = 29;
    pub const HANDLED_TELEPORT: i32 = 30;
    pub const MISSED_SWING: i32 = 31;
    pub const START_CRAWLING: i32 = 32;
    pub const STOP_CRAWLING: i32 = 33;
    pub const START_FLYING: i32 = 34;
    pub const STOP_FLYING: i32 = 35;

    pub const START_USING_ITEM: i32 = 37;

    pub const CRACK_BREAK: i32 = 18;
}
