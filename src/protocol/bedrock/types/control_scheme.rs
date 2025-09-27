#[derive(Debug)]
pub struct ControlScheme {}

impl ControlScheme {
    pub const LOCKED_PLAYER_RELATIVE_STRAFE: u8 = 0;
    pub const CAMERA_RELATIVE: u8 = 1;
    pub const CAMERA_RELATIVE_STRAFE: u8 = 2;
    pub const PLAYER_RELATIVE: u8 = 3;
    pub const PLAYER_RELATIVE_STRAFE: u8 = 4;
}