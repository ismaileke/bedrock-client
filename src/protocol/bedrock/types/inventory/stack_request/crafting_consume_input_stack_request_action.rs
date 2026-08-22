use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct CraftingConsumeInputStackRequestAction {}

impl CraftingConsumeInputStackRequestAction {
    pub fn new() -> CraftingConsumeInputStackRequestAction {
        CraftingConsumeInputStackRequestAction {}
    }

    pub fn read(_stream: &mut Reader) -> CraftingConsumeInputStackRequestAction {
        CraftingConsumeInputStackRequestAction {}
    }

    pub fn write(&self, _stream: &mut Writer) {}
}
