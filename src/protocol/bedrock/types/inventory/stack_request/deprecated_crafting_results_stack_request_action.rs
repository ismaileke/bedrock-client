use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::inventory::item_stack::ItemStack;
use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct DeprecatedCraftingResultsStackRequestAction {
    results: Vec<ItemStack>,
    iterations: u8,
}

impl DeprecatedCraftingResultsStackRequestAction {
    pub fn new(
        results: Vec<ItemStack>,
        iterations: u8,
    ) -> DeprecatedCraftingResultsStackRequestAction {
        DeprecatedCraftingResultsStackRequestAction {
            results,
            iterations,
        }
    }

    pub fn read(stream: &mut Stream) -> DeprecatedCraftingResultsStackRequestAction {
        let mut results = Vec::new();
        let len = stream.get_var_u32();
        for _ in 0..len {
            results.push(PacketSerializer::get_item_stack_without_stack_id(stream));
        }
        let iterations = stream.get_byte();

        DeprecatedCraftingResultsStackRequestAction {
            results,
            iterations,
        }
    }

    pub fn write(&mut self, stream: &mut Stream) {
        stream.put_var_u32(self.results.len() as u32);
        for result in &self.results {
            PacketSerializer::put_item_stack_without_stack_id(stream, result);
        }
        stream.put_byte(self.iterations);
    }
}
