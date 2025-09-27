pub struct SubChunkRequestResult {}

impl SubChunkRequestResult {
    pub const SUCCESS: u8 = 1;
    pub const NO_SUCH_CHUNK: u8 = 2;
    pub const WRONG_DIMENSION: u8 = 3;
    pub const NULL_PLAYER: u8 = 4;
    pub const Y_INDEX_OUT_OF_BOUNDS: u8 = 5;
    pub const SUCCESS_ALL_AIR: u8 = 6;
}
