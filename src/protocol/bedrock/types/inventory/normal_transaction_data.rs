use binary_utils::binary::Stream;
use crate::protocol::bedrock::inventory_transaction::InventoryTransaction;
use crate::protocol::bedrock::types::inventory::network_inventory_action::NetworkInventoryAction;
use crate::protocol::bedrock::types::inventory::transaction_data::TransactionData;

#[derive(Debug)]
pub struct NormalTransactionData {
    actions: Vec<NetworkInventoryAction>
}

impl NormalTransactionData {
    pub fn new(actions: Vec<NetworkInventoryAction>) -> NormalTransactionData {
        NormalTransactionData{ actions }
    }
}

impl TransactionData for NormalTransactionData {
    fn get_type_id(&self) -> u32 {
        InventoryTransaction::TYPE_NORMAL
    }

    fn get_actions(&self) -> &Vec<NetworkInventoryAction> {
        self.actions.as_ref()
    }

    fn get_actions_mut(&mut self) -> &mut Vec<NetworkInventoryAction> {
        self.actions.as_mut()
    }

    fn decode_data(&mut self, _stream: &mut Stream) {}

    fn encode_data(&self, _stream: &mut Stream) {}
}