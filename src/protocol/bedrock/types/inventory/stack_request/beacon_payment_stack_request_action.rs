use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct BeaconPaymentStackRequestAction {
    primary_effect_id: i32,
    secondary_effect_id: i32,
}

impl BeaconPaymentStackRequestAction {
    pub fn new(
        primary_effect_id: i32,
        secondary_effect_id: i32,
    ) -> BeaconPaymentStackRequestAction {
        BeaconPaymentStackRequestAction {
            primary_effect_id,
            secondary_effect_id,
        }
    }

    pub fn read(stream: &mut Stream) -> BeaconPaymentStackRequestAction {
        let primary_effect_id = stream.get_var_i32();
        let secondary_effect_id = stream.get_var_i32();

        BeaconPaymentStackRequestAction {
            primary_effect_id,
            secondary_effect_id,
        }
    }

    pub fn write(&mut self, stream: &mut Stream) {
        stream.put_var_i32(self.primary_effect_id);
        stream.put_var_i32(self.secondary_effect_id);
    }
}
