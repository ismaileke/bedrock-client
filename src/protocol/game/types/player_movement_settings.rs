use binary_utils::binary::Stream;

#[derive(Debug)]
pub struct PlayerMovementSettings {
    pub movement_type: i32,
    pub rewind_history_size: i32,
    pub server_auth_block_breaking: bool
}

impl PlayerMovementSettings {
    pub fn read(stream: &mut Stream) -> PlayerMovementSettings {
        let movement_type = stream.get_var_int();
        let rewind_history_size = stream.get_var_int();
        let server_auth_block_breaking = stream.get_bool();

        PlayerMovementSettings{ movement_type, rewind_history_size, server_auth_block_breaking }
    }
}