pub struct ContainerIds {}

impl ContainerIds {
    pub const NONE: u8 = u8::MAX; //-1?
    pub const INVENTORY: u8 = 0;
    pub const FIRST: u8 = 1;
    pub const LAST: u8 = 100;
    pub const OFFHAND: u8 = 119;
    pub const ARMOR: u8 = 120;

    pub const HOTBAR: u8 = 122;
    pub const FIXED_INVENTORY: u8 = 123;
    pub const UI: u8 = 124;
    pub const CONTAINER_ID_REGISTRY: u8 = 125;
}
