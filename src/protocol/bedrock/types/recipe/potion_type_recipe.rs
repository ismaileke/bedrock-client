#[derive(Debug)]
pub struct PotionTypeRecipe {
    pub input_item_id: i32,
    pub input_item_meta: i32,
    pub ingredient_item_id: i32,
    pub ingredient_item_meta: i32,
    pub output_item_id: i32,
    pub output_item_meta: i32
}

impl PotionTypeRecipe {
    pub fn get_input_item_id(&self) -> i32 {
        self.input_item_id
    }

    pub fn get_input_item_meta(&self) -> i32 {
        self.input_item_meta
    }

    pub fn get_ingredient_item_id(&self) -> i32 {
        self.ingredient_item_id
    }

    pub fn get_ingredient_item_meta(&self) -> i32 {
        self.ingredient_item_meta
    }

    pub fn get_output_item_id(&self) -> i32 {
        self.output_item_id
    }
    
    pub fn get_output_item_meta(&self) -> i32 {
        self.output_item_meta
    }
}
