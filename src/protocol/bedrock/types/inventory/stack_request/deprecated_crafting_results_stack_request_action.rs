use crate::protocol::bedrock::types::inventory::stack_request::item_stack_request_network_item_instance_descriptor::ItemStackRequestNetworkItemInstanceDescriptor;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct DeprecatedCraftingResultsStackRequestAction {
    results: Vec<ItemStackRequestNetworkItemInstanceDescriptor>,
    iterations: u8,
}

impl DeprecatedCraftingResultsStackRequestAction {
    pub fn new(results: Vec<ItemStackRequestNetworkItemInstanceDescriptor>, iterations: u8) -> DeprecatedCraftingResultsStackRequestAction {
        DeprecatedCraftingResultsStackRequestAction { results, iterations }
    }

    pub fn read(stream: &mut Reader) -> DeprecatedCraftingResultsStackRequestAction {
        let mut results = Vec::new();
        let len = stream.get_var_u32();
        for _ in 0..len {
            results.push(ItemStackRequestNetworkItemInstanceDescriptor::read(stream));
        }
        let iterations = stream.get_u8();

        DeprecatedCraftingResultsStackRequestAction { results, iterations }
    }

    pub fn write(&self, stream: &mut Writer) {
        stream.put_var_u32(self.results.len() as u32);
        for result in &self.results {
            result.write(stream);
        }
        stream.put_u8(self.iterations);
    }
}
