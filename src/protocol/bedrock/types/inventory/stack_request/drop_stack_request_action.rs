use binary_utils::binary::Stream;
use crate::protocol::bedrock::types::inventory::stack_request::item_stack_request_slot_info::ItemStackRequestSlotInfo;

#[derive(serde::Serialize, Debug)]
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

    pub fn write(&mut self, stream: &mut Stream) {
        stream.put_byte(self.count);
        self.source.write(stream);
        stream.put_bool(self.randomly);
    }
}
