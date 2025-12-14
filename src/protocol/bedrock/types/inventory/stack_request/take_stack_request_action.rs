use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct TakeStackRequestAction {}

impl TakeStackRequestAction {
    pub fn new() -> TakeStackRequestAction {
        TakeStackRequestAction {}
    }

    pub fn read(_stream: &mut Stream) -> TakeStackRequestAction {
        TakeStackRequestAction {}
    }

    pub fn write(&mut self, _stream: &mut Stream) {}
}
