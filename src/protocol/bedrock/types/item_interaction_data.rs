use binary_utils::binary::Stream;
use crate::protocol::bedrock::types::inventory::inventory_transaction_changed_slots_hack::InventoryTransactionChangedSlotsHack;
use crate::protocol::bedrock::types::inventory::item_stack::ItemStack;
use crate::protocol::bedrock::types::inventory::item_stack_wrapper::ItemStackWrapper;
use crate::protocol::bedrock::types::inventory::transaction_data::TransactionData;
use crate::protocol::bedrock::types::inventory::use_item_transaction_data::UseItemTransactionData;

#[derive(serde::Serialize, Debug)]
pub struct ItemInteractionData {
    request_id: i32,
    request_changed_slots: Vec<InventoryTransactionChangedSlotsHack>,
    tr_data: UseItemTransactionData
}

impl ItemInteractionData {
    pub fn new(request_id: i32, request_changed_slots: Vec<InventoryTransactionChangedSlotsHack>, tr_data: UseItemTransactionData) -> ItemInteractionData {
        ItemInteractionData{ request_id, request_changed_slots, tr_data }
    }

    pub fn read(stream: &mut Stream) -> ItemInteractionData {
        let request_id = stream.get_var_i32();
        let mut request_changed_slots = Vec::new();
        if request_id != 0 {
            let len = stream.get_var_u32();
            for _ in 0..len {
                request_changed_slots.push(InventoryTransactionChangedSlotsHack::read(stream));
            }
        }
        let mut tr_data = TransactionData::UseItem(UseItemTransactionData::new(vec![], 0, 0, vec![], 0, 0, ItemStackWrapper{ stack_id: 0, item_stack: ItemStack::null() }, vec![], vec![], 0, 0)); // bad way LOL
        tr_data.decode(stream);

        let use_item_tr_data = match tr_data {
            TransactionData::UseItem(data) => { data },
            _ => { panic!("Expected UseItemTransactionData, got {:?}", tr_data.get_type_id()) }
        };

        ItemInteractionData{ request_id, request_changed_slots, tr_data: use_item_tr_data }
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_var_i32(self.request_id);
        if self.request_id != 0 {
            stream.put_var_u32(self.request_changed_slots.len() as u32);
            for slots in self.request_changed_slots.iter() {
                slots.write(stream);
            }
        }
        let tr_data = TransactionData::UseItem(self.tr_data.clone());
        tr_data.encode(stream);
    }
}
