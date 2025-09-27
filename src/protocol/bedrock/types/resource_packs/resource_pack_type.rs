pub struct ResourcePackType {}

impl ResourcePackType {
    pub const INVALID: u8 = 0;
    pub const ADDON: u8 = 1;
    pub const CACHED: u8 = 2;
    pub const COPY_PROTECTED: u8 = 3;
    pub const BEHAVIORS: u8 = 4;
    pub const PERSONA_PIECE: u8 = 5;
    pub const RESOURCES: u8 = 6;
    pub const SKINS: u8 = 7;
    pub const WORLD_TEMPLATE: u8 = 8;
}