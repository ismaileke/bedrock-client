use crate::protocol::game::types::cacheable_nbt::CacheableNBT;

pub struct BlockPaletteEntry {
    name: String,
    states: CacheableNBT
}

impl BlockPaletteEntry {

    pub fn new(name: String, states: CacheableNBT) -> BlockPaletteEntry {
        BlockPaletteEntry { name, states }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_states(&self) -> &CacheableNBT {
        &self.states
    }
}