use crate::protocol::bedrock::types::recipe::material_reducer_recipe_output::MaterialReducerRecipeOutput;

#[derive(serde::Serialize, Debug)]
pub struct MaterialReducerRecipe {
    pub input_item_id: i32,
    pub input_item_meta: i32,
    pub outputs: Vec<MaterialReducerRecipeOutput>,
}

impl MaterialReducerRecipe {
    pub fn get_input_item_id(&self) -> i32 {
        self.input_item_id
    }

    pub fn get_input_item_meta(&self) -> i32 {
        self.input_item_meta
    }

    pub fn get_outputs(&self) -> &Vec<MaterialReducerRecipeOutput> {
        &self.outputs
    }
}
