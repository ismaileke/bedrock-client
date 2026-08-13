use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct NetworkPermissions {
    pub disable_client_sounds: bool,
}

impl NetworkPermissions {
    pub fn read(stream: &mut Reader) -> NetworkPermissions {
        let disable_client_sounds = stream.get_bool();

        NetworkPermissions { disable_client_sounds }
    }

    pub fn write(&self, stream: &mut Writer) {
        stream.put_bool(self.disable_client_sounds);
    }
}
