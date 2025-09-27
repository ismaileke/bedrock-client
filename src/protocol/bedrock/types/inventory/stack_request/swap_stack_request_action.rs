use binary_utils::binary::Stream;
use crate::protocol::bedrock::types::inventory::stack_request::item_stack_request_action::ItemStackRequestAction;
use crate::protocol::bedrock::types::inventory::stack_request::item_stack_request_action_type::ItemStackRequestActionType;
use crate::protocol::bedrock::types::inventory::stack_request::item_stack_request_slot_info::ItemStackRequestSlotInfo;

#[derive(Debug)]
pub struct SwapStackRequestAction {
    slot1: ItemStackRequestSlotInfo,
    slot2: ItemStackRequestSlotInfo
}

impl SwapStackRequestAction {
    pub fn new(slot1: ItemStackRequestSlotInfo, slot2: ItemStackRequestSlotInfo) -> SwapStackRequestAction {
        SwapStackRequestAction{ slot1, slot2 }
    }

    pub fn read(stream: &mut Stream) -> SwapStackRequestAction {
        let slot1 = ItemStackRequestSlotInfo::read(stream);
        let slot2 = ItemStackRequestSlotInfo::read(stream);

        SwapStackRequestAction{ slot1, slot2 }
    }
}

impl ItemStackRequestAction for SwapStackRequestAction {
    fn get_type_id(&self) -> u8 {
        ItemStackRequestActionType::SWAP
    }

    fn write(&mut self, stream: &mut Stream) {
        self.slot1.write(stream);
        self.slot2.write(stream);
    }
}


