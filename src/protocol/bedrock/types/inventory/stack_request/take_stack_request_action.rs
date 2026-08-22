use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct TakeStackRequestAction {}

impl TakeStackRequestAction {
    pub fn new() -> TakeStackRequestAction {
        TakeStackRequestAction {}
    }

    pub fn read(_stream: &mut Reader) -> TakeStackRequestAction {
        TakeStackRequestAction {}
    }

    pub fn write(&self, _stream: &mut Writer) {}
}
