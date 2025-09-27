pub struct OverrideUpdateType {}

impl OverrideUpdateType {
    pub const CLEAR_OVERRIDES: u8 = 0;
    pub const REMOVE_OVERRIDE: u8 = 1;
    pub const SET_INT_OVERRIDE: u8 = 2;
    pub const SET_FLOAT_OVERRIDE: u8 = 3;
}
