use crate::protocol::bedrock::types::recipe::item_descriptor::ItemDescriptor;

#[derive(serde::Serialize, Debug)]
pub struct RecipeIngredient{
    pub descriptor: Option<ItemDescriptor>,
    pub count: i32
}
