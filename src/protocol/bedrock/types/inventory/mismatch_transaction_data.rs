use binary_utils::binary::Stream;
use crate::protocol::bedrock::inventory_transaction::InventoryTransaction;
use crate::protocol::bedrock::types::inventory::network_inventory_action::NetworkInventoryAction;
use crate::protocol::bedrock::types::inventory::transaction_data::TransactionData;

#[derive(Debug)]
pub struct MismatchTransactionData {
    actions: Vec<NetworkInventoryAction>
}

impl MismatchTransactionData {
    pub fn new() -> MismatchTransactionData {
        MismatchTransactionData{ actions: Vec::new() }
    }
}

impl TransactionData for MismatchTransactionData {
    fn get_type_id(&self) -> u32 {
        InventoryTransaction::TYPE_MISMATCH
    }

    fn get_actions(&self) -> &Vec<NetworkInventoryAction> {
        self.actions.as_ref()
    }

    fn get_actions_mut(&mut self) -> &mut Vec<NetworkInventoryAction> {
        self.actions.as_mut()
    }

    fn decode_data(&mut self, _stream: &mut Stream) {
        if self.actions.len() > 0 {
            panic!("Mismatch transaction type should not have any actions associated with it, but got {}", self.actions.len());
        }
    }

    fn encode_data(&self, _stream: &mut Stream) {}
}