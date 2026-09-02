use crate::protocol::bedrock::types::inventory::stack_request::item_stack_request_slot_info::ItemStackRequestSlotInfo;
use binary_utils::binary::{Reader, Writer};

/// Bir yığından `count` adedini kaynaktan hedefe TAŞIR.
///
/// Alanlar (count, source, destination) protokolde hep vardı ama burada boş
/// bir struct duruyordu: gönderilen her Take/Place eylemi eksik yazılıyor ve
/// sunucu paketin geri kalanını çözemiyordu.
#[derive(serde::Serialize, Debug)]
pub struct TakeStackRequestAction {
    pub count: u8,
    pub source: ItemStackRequestSlotInfo,
    pub destination: ItemStackRequestSlotInfo,
}

impl TakeStackRequestAction {
    pub fn new(
        count: u8,
        source: ItemStackRequestSlotInfo,
        destination: ItemStackRequestSlotInfo,
    ) -> TakeStackRequestAction {
        TakeStackRequestAction { count, source, destination }
    }

    pub fn read(stream: &mut Reader) -> TakeStackRequestAction {
        let count = stream.get_u8();
        let source = ItemStackRequestSlotInfo::read(stream);
        let destination = ItemStackRequestSlotInfo::read(stream);

        TakeStackRequestAction { count, source, destination }
    }

    pub fn write(&self, stream: &mut Writer) {
        stream.put_u8(self.count);
        self.source.write(stream);
        self.destination.write(stream);
    }
}
