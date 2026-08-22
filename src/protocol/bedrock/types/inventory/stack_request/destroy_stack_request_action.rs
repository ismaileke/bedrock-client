use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct DestroyStackRequestAction {}

impl DestroyStackRequestAction {
    pub fn new() -> DestroyStackRequestAction {
        DestroyStackRequestAction {}
    }

    pub fn read(_stream: &mut Reader) -> DestroyStackRequestAction {
        DestroyStackRequestAction {}
    }

    pub fn write(&self, _stream: &mut Writer) {}
}
