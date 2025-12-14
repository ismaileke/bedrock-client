use crate::protocol::bedrock::types::inventory::network_inventory_action::NetworkInventoryAction;
use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct NormalTransactionData {
    actions: Vec<NetworkInventoryAction>,
}

impl NormalTransactionData {
    pub fn new(actions: Vec<NetworkInventoryAction>) -> NormalTransactionData {
        NormalTransactionData { actions }
    }

    pub fn get_actions(&self) -> &Vec<NetworkInventoryAction> {
        self.actions.as_ref()
    }

    pub fn get_actions_mut(&mut self) -> &mut Vec<NetworkInventoryAction> {
        self.actions.as_mut()
    }

    pub fn decode_data(&mut self, _stream: &mut Stream) {}

    pub fn encode_data(&self, _stream: &mut Stream) {}
}
