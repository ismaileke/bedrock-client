use binary_utils::binary::Stream;

pub struct PlayerMovementSettings {
    movement_type: i32,
    rewind_history_size: i32,
    server_auth_block_breaking: bool
}

impl PlayerMovementSettings {
    pub fn read(stream: &mut Stream) -> PlayerMovementSettings {
        let movement_type = stream.get_var_int();
        let rewind_history_size = stream.get_var_int();
        let server_auth_block_breaking = stream.get_bool();

        PlayerMovementSettings{ movement_type, rewind_history_size, server_auth_block_breaking }
    }
}