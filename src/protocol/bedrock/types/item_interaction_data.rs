use crate::protocol::bedrock::types::inventory::inventory_transaction_changed_slots_hack::InventoryTransactionChangedSlotsHack;
use crate::protocol::bedrock::types::inventory::transaction_data::TransactionData;
use crate::protocol::bedrock::types::inventory::use_item_transaction_data::UseItemTransactionData;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct ItemInteractionData {
    pub request_id: i32,
    pub request_changed_slots: Vec<InventoryTransactionChangedSlotsHack>,
    pub tr_data: UseItemTransactionData,
}

impl ItemInteractionData {
    pub fn new(
        request_id: i32,
        request_changed_slots: Vec<InventoryTransactionChangedSlotsHack>,
        tr_data: UseItemTransactionData,
    ) -> ItemInteractionData {
        ItemInteractionData { request_id, request_changed_slots, tr_data }
    }

    fn has_changed_slots(request_id: i32) -> bool {
        request_id < -1 && (request_id & 1) == 0
    }

    pub fn read(stream: &mut Reader) -> ItemInteractionData {
        let request_id = stream.get_var_i32();
        let mut request_changed_slots = Vec::new();
        if stream.get_bool() && Self::has_changed_slots(request_id) {
            let len = stream.get_var_u32();
            for _ in 0..len {
                request_changed_slots.push(InventoryTransactionChangedSlotsHack::read(stream));
            }
        }
        let mut tr_data = TransactionData::UseItem(UseItemTransactionData::null());
        tr_data.decode(stream);

        let use_item_tr_data = match tr_data {
            TransactionData::UseItem(data) => data,
            _ => panic!("Expected UseItemTransactionData, got {:?}", tr_data.get_type_id())
        };

        ItemInteractionData {
            request_id,
            request_changed_slots,
            tr_data: use_item_tr_data,
        }
    }

    pub fn write(&self, stream: &mut Writer) {
        stream.put_var_i32(self.request_id);
        let has_changed_slots = Self::has_changed_slots(self.request_id);
        stream.put_bool(has_changed_slots);
        if has_changed_slots {
            stream.put_var_u32(self.request_changed_slots.len() as u32);
            for slots in self.request_changed_slots.iter() {
                slots.write(stream);
            }
        }
        let tr_data = TransactionData::UseItem(self.tr_data.clone());
        tr_data.encode(stream);
    }
}
