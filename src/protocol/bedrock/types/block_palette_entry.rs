use crate::protocol::bedrock::types::cacheable_nbt::CacheableNBT;

#[derive(serde::Serialize, Debug, Clone)]
pub struct BlockPaletteEntry {
    pub name: String,
    pub states: CacheableNBT,
}

impl BlockPaletteEntry {
    pub fn new(name: String, states: CacheableNBT) -> BlockPaletteEntry {
        BlockPaletteEntry { name, states }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_states(&self) -> &CacheableNBT {
        &self.states
    }
}
