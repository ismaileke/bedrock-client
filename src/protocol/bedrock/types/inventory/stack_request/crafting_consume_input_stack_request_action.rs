use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct CraftingConsumeInputStackRequestAction {}

impl CraftingConsumeInputStackRequestAction {
    pub fn new() -> CraftingConsumeInputStackRequestAction {
        CraftingConsumeInputStackRequestAction {}
    }

    pub fn read(_stream: &mut Stream) -> CraftingConsumeInputStackRequestAction {
        CraftingConsumeInputStackRequestAction {}
    }

    pub fn write(&mut self, _stream: &mut Stream) {}
}
