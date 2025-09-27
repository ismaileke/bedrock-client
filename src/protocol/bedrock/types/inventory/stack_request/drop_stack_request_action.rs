use binary_utils::binary::Stream;
use crate::protocol::bedrock::types::inventory::stack_request::item_stack_request_action::ItemStackRequestAction;
use crate::protocol::bedrock::types::inventory::stack_request::item_stack_request_action_type::ItemStackRequestActionType;
use crate::protocol::bedrock::types::inventory::stack_request::item_stack_request_slot_info::ItemStackRequestSlotInfo;

#[derive(Debug)]
pub struct DropStackRequestAction {
    count: u8,
    source: ItemStackRequestSlotInfo,
    randomly: bool
}

impl DropStackRequestAction {
    pub fn new(count: u8, source: ItemStackRequestSlotInfo, randomly: bool) -> DropStackRequestAction {
        DropStackRequestAction{ count, source, randomly }
    }

    pub fn read(stream: &mut Stream) -> DropStackRequestAction {
        let count = stream.get_byte();
        let source = ItemStackRequestSlotInfo::read(stream);
        let randomly = stream.get_bool();

        DropStackRequestAction{ count, source, randomly }
    }
}

impl ItemStackRequestAction for DropStackRequestAction {
    fn get_type_id(&self) -> u8 {
        ItemStackRequestActionType::DROP
    }

    fn write(&mut self, stream: &mut Stream) {
        stream.put_byte(self.count);
        self.source.write(stream);
        stream.put_bool(self.randomly);
    }
}


