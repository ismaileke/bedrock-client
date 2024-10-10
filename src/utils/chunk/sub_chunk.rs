use crate::utils::chunk::paletted_storage::PalettedStorage;

pub struct SubChunk {
    pub air: u32,
    pub storages: Vec<PalettedStorage>,
    pub block_light: Vec<u8>,
    pub sky_light: Vec<u8>
}

impl SubChunk {
    pub fn new(air: u32) -> SubChunk {
        SubChunk{
            air,
            storages: vec![],
            block_light: vec![],
            sky_light: vec![],
        }
    }
}