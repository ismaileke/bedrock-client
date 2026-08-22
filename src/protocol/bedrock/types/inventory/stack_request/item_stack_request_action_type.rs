pub struct ItemStackRequestActionType {}

impl ItemStackRequestActionType {
    pub const TAKE: u32 = 0;
    pub const PLACE: u32 = 1;
    pub const SWAP: u32 = 2;
    pub const DROP: u32 = 3;
    pub const DESTROY: u32 = 4;
    pub const CRAFTING_CONSUME_INPUT: u32 = 5;
    pub const CRAFTING_CREATE_SPECIFIC_RESULT: u32 = 6;
    pub const LAB_TABLE_COMBINE: u32 = 7;
    pub const BEACON_PAYMENT: u32 = 8;
    pub const MINE_BLOCK: u32 = 9;
    pub const CRAFTING_RECIPE: u32 = 10;
    pub const CRAFTING_RECIPE_AUTO: u32 = 11; //recipe book?
    pub const CREATIVE_CREATE: u32 = 12;
    pub const CRAFTING_RECIPE_OPTIONAL: u32 = 13; //anvil/cartography table rename
    pub const CRAFTING_GRINDSTONE: u32 = 14;
    pub const CRAFTING_LOOM: u32 = 15;
    pub const CRAFTING_NON_IMPLEMENTED_DEPRECATED_ASK_TY_LAING: u32 = 16;
    pub const CRAFTING_RESULTS_DEPRECATED_ASK_TY_LAING: u32 = 17; //no idea what this is for

    /**
     * The legacy IDs still contain the gap left by the removal of PLACE_IN_ITEM_CONTAINER (7) and
     * TAKE_FROM_ITEM_CONTAINER (8), so everything from LAB_TABLE_COMBINE onwards is offset by 2.
     */
    pub fn legacy_type_id(type_id: u32) -> u32 {
        if type_id >= Self::LAB_TABLE_COMBINE { type_id + 2 } else { type_id }
    }
}
