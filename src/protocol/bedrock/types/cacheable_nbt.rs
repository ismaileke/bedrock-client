use mojang_nbt::nbt_serializer::NBTSerializer;
use mojang_nbt::tag::tag::Tag;
use mojang_nbt::tree_root::TreeRoot;
use std::fmt::Debug;

#[derive(Debug, serde::Serialize)]
pub struct CacheableNBT {
    nbt_root: Tag,
    encoded_nbt: Option<Vec<u8>>,
}

impl CacheableNBT {
    pub fn new(nbt_root: Tag) -> Self {
        CacheableNBT {
            nbt_root,
            encoded_nbt: None,
        }
    }

    pub fn get_root(&self) -> Tag {
        self.nbt_root.clone()
    }

    pub fn get_encoded_nbt(&mut self) -> Vec<u8> {
        if let None = self.encoded_nbt.as_ref() {
            let mut serializer = NBTSerializer::new_network();
            let encoded_nbt_vector =
                serializer.write(TreeRoot::new(self.get_root(), "".to_string()));

            self.encoded_nbt = Option::from(encoded_nbt_vector);
        }

        self.encoded_nbt.clone().unwrap()
    }
}
