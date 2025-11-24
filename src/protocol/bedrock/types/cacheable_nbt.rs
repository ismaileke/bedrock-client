use crate::protocol::bedrock::serializer::network_nbt_serializer::NetworkNBTSerializer;
use mojang_nbt::base_nbt_serializer::BaseNBTSerializer;
use mojang_nbt::tag::tag::Tag;
use mojang_nbt::tree_root::TreeRoot;
use std::fmt::Debug;

#[derive(serde::Serialize)]
pub struct CacheableNBT {
    #[serde(skip)]
    nbt_root: Box<dyn Tag>,
    encoded_nbt: Option<Vec<u8>>
}

impl Debug for CacheableNBT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CacheableNBT")
            .field("nbt_root", &self.nbt_root.get_value())
            .field("encoded_nbt", &self.encoded_nbt)
            .finish()
    }
}

impl CacheableNBT {
    pub fn new(nbt_root: Box<dyn Tag>) -> Self {
        CacheableNBT{ nbt_root, encoded_nbt: None }
    }

    pub fn get_root(&self) -> Box<dyn Tag> {
        self.nbt_root.clone_box()
    }

    pub fn get_encoded_nbt(&mut self) -> Vec<u8> {
        if let None = self.encoded_nbt.as_ref() {
            let mut serializer = NetworkNBTSerializer::new();
            let encoded_nbt_vector = serializer.write(TreeRoot::new(self.get_root(), "".to_string()));

            self.encoded_nbt = Option::from(encoded_nbt_vector);
        }

        self.encoded_nbt.clone().unwrap()
    }
}
