pub struct CommandPermissions {}

impl CommandPermissions {
    pub const NORMAL: u8 = 0;
    pub const OPERATOR: u8 = 1;
    pub const AUTOMATION: u8 = 2; //command blocks
    pub const HOST: u8 = 3; //hosting player on LAN multiplayer
    pub const OWNER: u8 = 4; //server terminal on BDS
    pub const INTERNAL: u8 = 5;
}
