use binary_utils::binary::Stream;
use crate::protocol::bedrock::types::inventory::stack_request::item_stack_request_action::ItemStackRequestAction;
use crate::protocol::bedrock::types::inventory::stack_request::item_stack_request_action_type::ItemStackRequestActionType;

#[derive(Debug)]
pub struct PlaceStackRequestAction {}

impl PlaceStackRequestAction {
    pub fn new() -> PlaceStackRequestAction {
        PlaceStackRequestAction{}
    }

    pub fn read(_stream: &mut Stream) -> PlaceStackRequestAction {
        PlaceStackRequestAction{}
    }
}

impl ItemStackRequestAction for PlaceStackRequestAction {
    fn get_type_id(&self) -> u8 {
        ItemStackRequestActionType::PLACE
    }

    fn write(&mut self, _stream: &mut Stream) {}
}


