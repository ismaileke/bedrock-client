use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::inventory::item_stack::ItemStack;
use crate::protocol::bedrock::types::inventory::stack_request::item_stack_request_action::ItemStackRequestAction;
use crate::protocol::bedrock::types::inventory::stack_request::item_stack_request_action_type::ItemStackRequestActionType;

#[derive(Debug)]
pub struct DeprecatedCraftingResultsStackRequestAction {
    results: Vec<ItemStack>,
    iterations: u8
}

impl DeprecatedCraftingResultsStackRequestAction {
    pub fn new(results: Vec<ItemStack>, iterations: u8) -> DeprecatedCraftingResultsStackRequestAction {
        DeprecatedCraftingResultsStackRequestAction{ results, iterations }
    }

    pub fn read(stream: &mut Stream) -> DeprecatedCraftingResultsStackRequestAction {
        let mut results = Vec::new();
        let len = stream.get_unsigned_var_int();
        for _ in 0..len {
            results.push(PacketSerializer::get_item_stack_without_stack_id(stream));
        }
        let iterations = stream.get_byte();

        DeprecatedCraftingResultsStackRequestAction{ results, iterations }
    }
}

impl ItemStackRequestAction for DeprecatedCraftingResultsStackRequestAction {
    fn get_type_id(&self) -> u8 {
        ItemStackRequestActionType::CRAFTING_RESULTS_DEPRECATED_ASK_TY_LAING
    }

    fn write(&mut self, stream: &mut Stream) {
        stream.put_unsigned_var_int(self.results.len() as u32);
        for result in &self.results {
            PacketSerializer::put_item_stack_without_stack_id(stream, result);
        }
        stream.put_byte(self.iterations);
    }
}


