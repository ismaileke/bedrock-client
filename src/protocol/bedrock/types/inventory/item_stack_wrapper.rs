use crate::protocol::bedrock::types::inventory::item_stack::ItemStack;

#[derive(serde::Serialize, Debug, Clone, Default)]
pub struct ItemStackWrapper {
    pub stack_id: i32,
    pub item_stack: ItemStack,
}
