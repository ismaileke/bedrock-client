pub enum AgentActionType {}

impl AgentActionType {
    pub const ATTACK: u32 = 1;
    pub const COLLECT: u32 = 2;
    pub const DESTROY: u32 = 3;
    pub const DETECT_REDSTONE: u32 = 4;
    pub const DETECT_OBSTACLE: u32 = 5;
    pub const DROP: u32 = 6;
    pub const DROP_ALL: u32 = 7;
    pub const INSPECT: u32 = 8;
    pub const INSPECT_DATA: u32 = 9;
    pub const INSPECT_ITEM_COUNT: u32 = 10;
    pub const INSPECT_ITEM_DETAIL: u32 = 11;
    pub const INSPECT_ITEM_SPACE: u32 = 12;
    pub const INTERACT: u32 = 13;
    pub const MOVE: u32 = 14;
    pub const PLACE_BLOCK: u32 = 15;
    pub const TILL: u32 = 16;
    pub const TRANSFER_ITEM_TO: u32 = 17;
    pub const TURN: u32 = 18;
}
