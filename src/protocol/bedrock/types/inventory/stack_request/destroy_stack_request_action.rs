use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct DestroyStackRequestAction {}

impl DestroyStackRequestAction {
    pub fn new() -> DestroyStackRequestAction {
        DestroyStackRequestAction {}
    }

    pub fn read(_stream: &mut Stream) -> DestroyStackRequestAction {
        DestroyStackRequestAction {}
    }

    pub fn write(&mut self, _stream: &mut Stream) {}
}
