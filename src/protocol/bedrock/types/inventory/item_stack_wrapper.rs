use crate::protocol::bedrock::types::inventory::item_stack::ItemStack;

#[derive(Debug, Clone)]
pub struct ItemStackWrapper {
    pub stack_id: i32,
    pub item_stack: ItemStack
}
