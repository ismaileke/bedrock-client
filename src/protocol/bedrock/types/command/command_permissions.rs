pub struct CommandPermissions {}

impl CommandPermissions {
    pub const NORMAL: u8 = 0;
    pub const OPERATOR: u8 = 1;
    pub const AUTOMATION: u8 = 2; //command blocks
    pub const HOST: u8 = 3; //hosting player on LAN multiplayer
    pub const OWNER: u8 = 4; //server terminal on BDS
    pub const INTERNAL: u8 = 5;
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum Permission {
    Any = 0,
    GameDirectors = 1,
    Admin = 2,
    Host = 3,
    Owner = 4,
    Internal = 5,
}

impl Permission {
    pub fn name(self) -> &'static str {
        match self {
            Permission::Any => "any",
            Permission::GameDirectors => "gamedirectors",
            Permission::Admin => "admin",
            Permission::Host => "host",
            Permission::Owner => "owner",
            Permission::Internal => "internal",
        }
    }

    pub fn from_name(name: &str) -> Self {
        match name {
            "any" => Permission::Any,
            "gamedirectors" => Permission::GameDirectors,
            "admin" => Permission::Admin,
            "host" => Permission::Host,
            "owner" => Permission::Owner,
            "internal" => Permission::Internal,
            _ => panic!("unknown permission {}", name),
        }
    }
}

