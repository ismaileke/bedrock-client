use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct DeprecatedCraftingNonImplementedStackRequestAction {}

impl DeprecatedCraftingNonImplementedStackRequestAction {
    pub fn new() -> DeprecatedCraftingNonImplementedStackRequestAction {
        DeprecatedCraftingNonImplementedStackRequestAction {}
    }

    pub fn read(_stream: &mut Stream) -> DeprecatedCraftingNonImplementedStackRequestAction {
        DeprecatedCraftingNonImplementedStackRequestAction {}
    }

    pub fn write(&mut self, _stream: &mut Stream) {}
}
