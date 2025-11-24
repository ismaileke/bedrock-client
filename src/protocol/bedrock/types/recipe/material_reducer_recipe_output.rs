#[derive(serde::Serialize, Debug)]
pub struct MaterialReducerRecipeOutput {
    pub item_id: i32,
    pub count: i32
}

impl MaterialReducerRecipeOutput {
    pub fn get_item_id(&self) -> i32 {
        self.item_id
    }

    pub fn get_count(&self) -> i32 {
        self.count
    }
}
