use binary_utils::binary::Stream;

pub struct NetworkPermissions {
    disable_client_sounds: bool
}

impl NetworkPermissions {
    pub fn read(stream: &mut Stream) -> NetworkPermissions {
        let disable_client_sounds = stream.get_bool();

        NetworkPermissions{ disable_client_sounds }
    }
}