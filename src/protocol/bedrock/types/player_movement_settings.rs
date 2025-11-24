use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct PlayerMovementSettings {
    pub rewind_history_size: i32,
    pub server_auth_block_breaking: bool
}

impl PlayerMovementSettings {
    pub fn read(stream: &mut Stream) -> PlayerMovementSettings {
        let rewind_history_size = stream.get_var_i32();
        let server_auth_block_breaking = stream.get_bool();

        PlayerMovementSettings{ rewind_history_size, server_auth_block_breaking }
    }
}