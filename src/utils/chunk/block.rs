use std::collections::HashMap;

pub struct BlockType {
    pub name: String,
    pub properties: HashMap<String, PropertyValues>,
    pub base_runtime_id: Option<u32>,
}

impl BlockType {
    pub fn new(name: String) -> Self {
        Self{
            name,
            properties: HashMap::new(),
            base_runtime_id: None,
        }
    }
}

#[derive(Clone)]
pub struct PropertyValues {
    pub strings: Vec<String>,
    pub bools: Vec<bool>,
    pub ints: Vec<u32>,
}