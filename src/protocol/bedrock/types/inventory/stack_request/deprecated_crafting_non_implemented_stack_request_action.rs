use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct DeprecatedCraftingNonImplementedStackRequestAction {}

impl DeprecatedCraftingNonImplementedStackRequestAction {
    pub fn new() -> DeprecatedCraftingNonImplementedStackRequestAction {
        DeprecatedCraftingNonImplementedStackRequestAction {}
    }

    pub fn read(_stream: &mut Reader) -> DeprecatedCraftingNonImplementedStackRequestAction {
        DeprecatedCraftingNonImplementedStackRequestAction {}
    }

    pub fn write(&mut self, _stream: &mut Writer) {}
}
