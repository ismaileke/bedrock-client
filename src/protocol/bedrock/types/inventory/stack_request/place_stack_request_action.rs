use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct PlaceStackRequestAction {}

impl PlaceStackRequestAction {
    pub fn new() -> PlaceStackRequestAction {
        PlaceStackRequestAction {}
    }

    pub fn read(_stream: &mut Stream) -> PlaceStackRequestAction {
        PlaceStackRequestAction {}
    }

    pub fn write(&mut self, _stream: &mut Stream) {}
}
