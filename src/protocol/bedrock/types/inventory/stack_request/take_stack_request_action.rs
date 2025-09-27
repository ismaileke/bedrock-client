use binary_utils::binary::Stream;
use crate::protocol::bedrock::types::inventory::stack_request::item_stack_request_action::ItemStackRequestAction;
use crate::protocol::bedrock::types::inventory::stack_request::item_stack_request_action_type::ItemStackRequestActionType;

#[derive(Debug)]
pub struct TakeStackRequestAction {}

impl TakeStackRequestAction {
    pub fn new() -> TakeStackRequestAction {
        TakeStackRequestAction{}
    }

    pub fn read(_stream: &mut Stream) -> TakeStackRequestAction {
        TakeStackRequestAction{}
    }
}

impl ItemStackRequestAction for TakeStackRequestAction {
    fn get_type_id(&self) -> u8 {
        ItemStackRequestActionType::TAKE
    }

    fn write(&mut self, _stream: &mut Stream) {}
}


