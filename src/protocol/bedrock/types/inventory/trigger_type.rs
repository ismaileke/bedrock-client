pub struct TriggerType {}

impl TriggerType {
    pub const UNKNOWN: u32 = 0;
    pub const PLAYER_INPUT: u32 = 1;
    pub const SIMULATION_TICK: u32 = 2;
}
