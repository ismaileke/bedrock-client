use binary_utils::binary::Stream;
use crate::protocol::bedrock::types::inventory::stack_request::item_stack_request_action::ItemStackRequestAction;
use crate::protocol::bedrock::types::inventory::stack_request::item_stack_request_action_type::ItemStackRequestActionType;

#[derive(Debug)]
pub struct BeaconPaymentStackRequestAction {
    primary_effect_id: i32,
    secondary_effect_id: i32
}

impl BeaconPaymentStackRequestAction {
    pub fn new(primary_effect_id: i32, secondary_effect_id: i32) -> BeaconPaymentStackRequestAction {
        BeaconPaymentStackRequestAction{ primary_effect_id, secondary_effect_id }
    }

    pub fn read(stream: &mut Stream) -> BeaconPaymentStackRequestAction {
        let primary_effect_id = stream.get_var_int();
        let secondary_effect_id = stream.get_var_int();

        BeaconPaymentStackRequestAction{ primary_effect_id, secondary_effect_id }
    }
}

impl ItemStackRequestAction for BeaconPaymentStackRequestAction {
    fn get_type_id(&self) -> u8 {
        ItemStackRequestActionType::BEACON_PAYMENT
    }

    fn write(&mut self, stream: &mut Stream) {
        stream.put_var_int(self.primary_effect_id);
        stream.put_var_int(self.secondary_effect_id);
    }
}


