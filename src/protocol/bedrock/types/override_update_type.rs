pub struct OverrideUpdateType {}

impl OverrideUpdateType {
    pub const CLEAR_OVERRIDES: u32 = 0;
    pub const REMOVE_OVERRIDE: u32 = 1;
    pub const SET_INT_OVERRIDE: u32 = 2;
    pub const SET_FLOAT_OVERRIDE: u32 = 3;

    pub const NAMES: [&str; 4] = ["clearoverrides", "removeoverride", "setintoverride", "setfloatoverride"];
}
