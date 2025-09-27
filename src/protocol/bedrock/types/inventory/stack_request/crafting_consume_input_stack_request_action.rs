use binary_utils::binary::Stream;
use crate::protocol::bedrock::types::inventory::stack_request::item_stack_request_action::ItemStackRequestAction;
use crate::protocol::bedrock::types::inventory::stack_request::item_stack_request_action_type::ItemStackRequestActionType;

#[derive(Debug)]
pub struct CraftingConsumeInputStackRequestAction {}

impl CraftingConsumeInputStackRequestAction {
    pub fn new() -> CraftingConsumeInputStackRequestAction {
        CraftingConsumeInputStackRequestAction{}
    }

    pub fn read(_stream: &mut Stream) -> CraftingConsumeInputStackRequestAction {

        CraftingConsumeInputStackRequestAction{}
    }
}

impl ItemStackRequestAction for CraftingConsumeInputStackRequestAction {
    fn get_type_id(&self) -> u8 {
        ItemStackRequestActionType::CRAFTING_CONSUME_INPUT
    }

    fn write(&mut self, _stream: &mut Stream) {
    }
}


