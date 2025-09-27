pub struct SubChunkHeightMapType {}

impl SubChunkHeightMapType {
    pub const NO_DATA: u8 = 0;
    pub const DATA: u8 = 1;
    pub const ALL_TOO_HIGH: u8 = 2;
    pub const ALL_TOO_LOW: u8 = 3;
    pub const ALL_COPIED: u8 = 4;
}
