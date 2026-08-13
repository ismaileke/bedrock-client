use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct PlaceStackRequestAction {}

impl PlaceStackRequestAction {
    pub fn new() -> PlaceStackRequestAction {
        PlaceStackRequestAction {}
    }

    pub fn read(_stream: &mut Reader) -> PlaceStackRequestAction {
        PlaceStackRequestAction {}
    }

    pub fn write(&mut self, _stream: &mut Writer) {}
}
