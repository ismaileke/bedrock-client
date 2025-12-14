use crate::protocol::bedrock::types::inventory::network_inventory_action::NetworkInventoryAction;
use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct MismatchTransactionData {
    actions: Vec<NetworkInventoryAction>,
}

impl MismatchTransactionData {
    pub fn new() -> MismatchTransactionData {
        MismatchTransactionData {
            actions: Vec::new(),
        }
    }

    pub fn get_actions(&self) -> &Vec<NetworkInventoryAction> {
        self.actions.as_ref()
    }

    pub fn get_actions_mut(&mut self) -> &mut Vec<NetworkInventoryAction> {
        self.actions.as_mut()
    }

    pub fn decode_data(&mut self, _stream: &mut Stream) {
        if self.actions.len() > 0 {
            panic!("Mismatch transaction type should not have any actions associated with it, but got {}", self.actions.len());
        }
    }

    pub fn encode_data(&self, _stream: &mut Stream) {}
}
