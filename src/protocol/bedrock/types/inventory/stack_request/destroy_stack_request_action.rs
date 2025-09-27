use binary_utils::binary::Stream;
use crate::protocol::bedrock::types::inventory::stack_request::item_stack_request_action::ItemStackRequestAction;
use crate::protocol::bedrock::types::inventory::stack_request::item_stack_request_action_type::ItemStackRequestActionType;

#[derive(Debug)]
pub struct DestroyStackRequestAction {}

impl DestroyStackRequestAction {
    pub fn new() -> DestroyStackRequestAction {
        DestroyStackRequestAction{}
    }

    pub fn read(_stream: &mut Stream) -> DestroyStackRequestAction {
        DestroyStackRequestAction{}
    }
}

impl ItemStackRequestAction for DestroyStackRequestAction {
    fn get_type_id(&self) -> u8 {
        ItemStackRequestActionType::DESTROY
    }

    fn write(&mut self, _stream: &mut Stream) {}
}


