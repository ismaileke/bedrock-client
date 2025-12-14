#[derive(serde::Serialize, Debug)]
pub struct PotionContainerChangeRecipe {
    pub input_item_id: i32,
    pub ingredient_item_id: i32,
    pub output_item_id: i32,
}

impl PotionContainerChangeRecipe {
    pub fn input_item_id(&self) -> i32 {
        self.input_item_id
    }

    pub fn ingredient_item_id(&self) -> i32 {
        self.ingredient_item_id
    }

    pub fn output_item_id(&self) -> i32 {
        self.output_item_id
    }
}
