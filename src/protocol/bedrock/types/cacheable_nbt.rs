use mojang_nbt::nbt_serializer::NBTWriter;
use mojang_nbt::tag::tag::Tag;
use mojang_nbt::tree_root::TreeRoot;
use std::fmt::Debug;

#[derive(Debug, serde::Serialize, Clone)]
pub struct CacheableNBT {
    pub nbt_root: Tag,
    pub encoded_nbt: Option<Vec<u8>>,
}

impl CacheableNBT {
    pub fn new(nbt_root: Tag) -> Self {
        CacheableNBT {
            nbt_root,
            encoded_nbt: None,
        }
    }

    pub fn get_root(&self) -> &Tag {
        &self.nbt_root
    }

    pub fn get_encoded_nbt(&mut self) -> &[u8] {
        if self.encoded_nbt.is_none() {
            let mut serializer = NBTWriter::new_network();
            let bytes = serializer.write(TreeRoot::new(self.nbt_root.clone(), ""));
            self.encoded_nbt = Some(bytes.to_vec());
        }
        self.encoded_nbt.as_ref().unwrap()
    }
}
