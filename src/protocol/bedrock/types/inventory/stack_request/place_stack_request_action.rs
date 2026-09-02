use crate::protocol::bedrock::types::inventory::stack_request::item_stack_request_slot_info::ItemStackRequestSlotInfo;
use binary_utils::binary::{Reader, Writer};

/// Take ile aynı gövde: `count` adedi kaynaktan hedefe koyar.
#[derive(serde::Serialize, Debug)]
pub struct PlaceStackRequestAction {
    pub count: u8,
    pub source: ItemStackRequestSlotInfo,
    pub destination: ItemStackRequestSlotInfo,
}

impl PlaceStackRequestAction {
    pub fn new(
        count: u8,
        source: ItemStackRequestSlotInfo,
        destination: ItemStackRequestSlotInfo,
    ) -> PlaceStackRequestAction {
        PlaceStackRequestAction { count, source, destination }
    }

    pub fn read(stream: &mut Reader) -> PlaceStackRequestAction {
        let count = stream.get_u8();
        let source = ItemStackRequestSlotInfo::read(stream);
        let destination = ItemStackRequestSlotInfo::read(stream);

        PlaceStackRequestAction { count, source, destination }
    }

    pub fn write(&self, stream: &mut Writer) {
        stream.put_u8(self.count);
        self.source.write(stream);
        self.destination.write(stream);
    }
}
