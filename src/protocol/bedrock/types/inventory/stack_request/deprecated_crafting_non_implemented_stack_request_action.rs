use binary_utils::binary::Stream;
use crate::protocol::bedrock::types::inventory::stack_request::item_stack_request_action::ItemStackRequestAction;
use crate::protocol::bedrock::types::inventory::stack_request::item_stack_request_action_type::ItemStackRequestActionType;

#[derive(Debug)]
pub struct DeprecatedCraftingNonImplementedStackRequestAction {}

impl DeprecatedCraftingNonImplementedStackRequestAction {
    pub fn new() -> DeprecatedCraftingNonImplementedStackRequestAction {
        DeprecatedCraftingNonImplementedStackRequestAction{}
    }

    pub fn read(_stream: &mut Stream) -> DeprecatedCraftingNonImplementedStackRequestAction {
        DeprecatedCraftingNonImplementedStackRequestAction{}
    }
}

impl ItemStackRequestAction for DeprecatedCraftingNonImplementedStackRequestAction {
    fn get_type_id(&self) -> u8 {
        ItemStackRequestActionType::CRAFTING_NON_IMPLEMENTED_DEPRECATED_ASK_TY_LAING
    }

    fn write(&mut self, _stream: &mut Stream) {}
}


