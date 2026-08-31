use crate::protocol::bedrock::types::inventory::stack_request::item_stack_request_slot_info::ItemStackRequestSlotInfo;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct DropStackRequestAction {
    pub count: u8,
    pub source: ItemStackRequestSlotInfo,
    pub randomly: bool,
}

impl DropStackRequestAction {
    pub fn new(count: u8, source: ItemStackRequestSlotInfo, randomly: bool) -> DropStackRequestAction {
        DropStackRequestAction {
            count,
            source,
            randomly,
        }
    }

    pub fn read(stream: &mut Reader) -> DropStackRequestAction {
        let count = stream.get_u8();
        let source = ItemStackRequestSlotInfo::read(stream);
        let randomly = stream.get_bool();

        DropStackRequestAction { count, source, randomly }
    }

    pub fn write(&self, stream: &mut Writer) {
        stream.put_u8(self.count);
        self.source.write(stream);
        stream.put_bool(self.randomly);
    }
}
