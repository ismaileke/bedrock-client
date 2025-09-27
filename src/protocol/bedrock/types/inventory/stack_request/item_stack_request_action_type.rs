pub struct ItemStackRequestActionType {}

impl ItemStackRequestActionType {
    pub const TAKE: u8 = 0;
    pub const PLACE: u8 = 1;
    pub const SWAP: u8 = 2;
    pub const DROP: u8 = 3;
    pub const DESTROY: u8 = 4;
    pub const CRAFTING_CONSUME_INPUT: u8 = 5;
    pub const CRAFTING_CREATE_SPECIFIC_RESULT: u8 = 6;
    pub const LAB_TABLE_COMBINE: u8 = 9;
    pub const BEACON_PAYMENT: u8 = 10;
    pub const MINE_BLOCK: u8 = 11;
    pub const CRAFTING_RECIPE: u8 = 12;
    pub const CRAFTING_RECIPE_AUTO: u8 = 13; //recipe book?
    pub const CREATIVE_CREATE: u8 = 14;
    pub const CRAFTING_RECIPE_OPTIONAL: u8 = 15; //anvil/cartography table rename
    pub const CRAFTING_GRINDSTONE: u8 = 16;
    pub const CRAFTING_LOOM: u8 = 17;
    pub const CRAFTING_NON_IMPLEMENTED_DEPRECATED_ASK_TY_LAING: u8 = 18;
    pub const CRAFTING_RESULTS_DEPRECATED_ASK_TY_LAING: u8 = 19; //no idea what this is for
}