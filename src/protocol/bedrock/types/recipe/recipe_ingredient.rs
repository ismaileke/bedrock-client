use crate::protocol::bedrock::types::recipe::item_descriptor::ItemDescriptor;

#[derive(Debug)]
pub struct RecipeIngredient{
    pub descriptor: Option<Box<dyn ItemDescriptor>>,
    pub count: i32
}
