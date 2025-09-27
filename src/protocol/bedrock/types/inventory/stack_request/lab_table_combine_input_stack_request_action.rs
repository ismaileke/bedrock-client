use binary_utils::binary::Stream;
use crate::protocol::bedrock::types::inventory::stack_request::item_stack_request_action::ItemStackRequestAction;
use crate::protocol::bedrock::types::inventory::stack_request::item_stack_request_action_type::ItemStackRequestActionType;

#[derive(Debug)]
pub struct LabTableCombineInputStackRequestAction {}

impl LabTableCombineInputStackRequestAction {
    pub fn new() -> LabTableCombineInputStackRequestAction {
        LabTableCombineInputStackRequestAction{}
    }

    pub fn read(_stream: &mut Stream) -> LabTableCombineInputStackRequestAction {

        LabTableCombineInputStackRequestAction{}
    }
}

impl ItemStackRequestAction for LabTableCombineInputStackRequestAction {
    fn get_type_id(&self) -> u8 {
        ItemStackRequestActionType::LAB_TABLE_COMBINE
    }

    fn write(&mut self, _stream: &mut Stream) {
    }
}


