use std::fmt::Debug;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::types::inventory::network_inventory_action::NetworkInventoryAction;

pub trait TransactionData: Debug {
    fn get_type_id(&self) -> u32;
    fn get_actions(&self) -> &Vec<NetworkInventoryAction>;
    fn get_actions_mut(&mut self) -> &mut Vec<NetworkInventoryAction>;
    fn decode(&mut self, stream: &mut Stream) {
        let action_count = stream.get_unsigned_var_int();
        for _ in 0..action_count {
            let action = NetworkInventoryAction::read(stream);
            self.get_actions_mut().push(action);
        }
        self.decode_data(stream)
    }

    fn encode(&self, stream: &mut Stream) {
        stream.put_unsigned_var_int(self.get_actions().len() as u32);
        for action in self.get_actions() {
            action.write(stream);
        }
        self.encode_data(stream)
    }

    fn decode_data(&mut self, stream: &mut Stream);
    fn encode_data(&self, stream: &mut Stream);
}
