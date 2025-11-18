use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::inventory::stack_request::beacon_payment_stack_request_action::BeaconPaymentStackRequestAction;
use crate::protocol::bedrock::types::inventory::stack_request::craft_recipe_auto_stack_request_action::CraftRecipeAutoStackRequestAction;
use crate::protocol::bedrock::types::inventory::stack_request::craft_recipe_optional_stack_request_action::CraftRecipeOptionalStackRequestAction;
use crate::protocol::bedrock::types::inventory::stack_request::craft_recipe_stack_request_action::CraftRecipeStackRequestAction;
use crate::protocol::bedrock::types::inventory::stack_request::crafting_consume_input_stack_request_action::CraftingConsumeInputStackRequestAction;
use crate::protocol::bedrock::types::inventory::stack_request::creative_create_stack_request_action::CreativeCreateStackRequestAction;
use crate::protocol::bedrock::types::inventory::stack_request::deprecated_crafting_non_implemented_stack_request_action::DeprecatedCraftingNonImplementedStackRequestAction;
use crate::protocol::bedrock::types::inventory::stack_request::deprecated_crafting_results_stack_request_action::DeprecatedCraftingResultsStackRequestAction;
use crate::protocol::bedrock::types::inventory::stack_request::destroy_stack_request_action::DestroyStackRequestAction;
use crate::protocol::bedrock::types::inventory::stack_request::drop_stack_request_action::DropStackRequestAction;
use crate::protocol::bedrock::types::inventory::stack_request::grindstone_stack_request_action::GrindstoneStackRequestAction;
use crate::protocol::bedrock::types::inventory::stack_request::item_stack_request_action::ItemStackRequestAction;
use crate::protocol::bedrock::types::inventory::stack_request::item_stack_request_action_type::ItemStackRequestActionType;
use crate::protocol::bedrock::types::inventory::stack_request::lab_table_combine_input_stack_request_action::LabTableCombineInputStackRequestAction;
use crate::protocol::bedrock::types::inventory::stack_request::loom_stack_request_action::LoomStackRequestAction;
use crate::protocol::bedrock::types::inventory::stack_request::mine_block_stack_request_action::MineBlockStackRequestAction;
use crate::protocol::bedrock::types::inventory::stack_request::place_stack_request_action::PlaceStackRequestAction;
use crate::protocol::bedrock::types::inventory::stack_request::swap_stack_request_action::SwapStackRequestAction;
use crate::protocol::bedrock::types::inventory::stack_request::take_stack_request_action::TakeStackRequestAction;

#[derive(Debug)]
pub struct ItemStackRequestEntry {
    request_id: i32,
    actions: Vec<Box<dyn ItemStackRequestAction>>,
    filter_strings: Vec<String>,
    filter_string_cause: i32
}

impl ItemStackRequestEntry {
    pub fn new(request_id: i32, actions: Vec<Box<dyn ItemStackRequestAction>>, filter_strings: Vec<String>, filter_string_cause: i32) -> ItemStackRequestEntry {
        ItemStackRequestEntry{ request_id, actions, filter_strings, filter_string_cause }
    }

    fn read_action(stream: &mut Stream, type_id: u8) -> Box<dyn ItemStackRequestAction> {
        match type_id {
            ItemStackRequestActionType::TAKE => { Box::new(TakeStackRequestAction::read(stream)) as Box<dyn ItemStackRequestAction> }
            ItemStackRequestActionType::PLACE => { Box::new(PlaceStackRequestAction::read(stream)) as Box<dyn ItemStackRequestAction> }
            ItemStackRequestActionType::SWAP => { Box::new(SwapStackRequestAction::read(stream)) as Box<dyn ItemStackRequestAction> }
            ItemStackRequestActionType::DROP => { Box::new(DropStackRequestAction::read(stream)) as Box<dyn ItemStackRequestAction> }
            ItemStackRequestActionType::DESTROY => { Box::new(DestroyStackRequestAction::read(stream)) as Box<dyn ItemStackRequestAction> }
            ItemStackRequestActionType::CRAFTING_CONSUME_INPUT => { Box::new(CraftingConsumeInputStackRequestAction::read(stream)) as Box<dyn ItemStackRequestAction> }
            ItemStackRequestActionType::LAB_TABLE_COMBINE => { Box::new(LabTableCombineInputStackRequestAction::read(stream)) as Box<dyn ItemStackRequestAction> }
            ItemStackRequestActionType::BEACON_PAYMENT => { Box::new(BeaconPaymentStackRequestAction::read(stream)) as Box<dyn ItemStackRequestAction> }
            ItemStackRequestActionType::MINE_BLOCK => { Box::new(MineBlockStackRequestAction::read(stream)) as Box<dyn ItemStackRequestAction> }
            ItemStackRequestActionType::CRAFTING_RECIPE => { Box::new(CraftRecipeStackRequestAction::read(stream)) as Box<dyn ItemStackRequestAction> }
            ItemStackRequestActionType::CRAFTING_RECIPE_AUTO => { Box::new(CraftRecipeAutoStackRequestAction::read(stream)) as Box<dyn ItemStackRequestAction> }
            ItemStackRequestActionType::CREATIVE_CREATE => { Box::new(CreativeCreateStackRequestAction::read(stream)) as Box<dyn ItemStackRequestAction> }
            ItemStackRequestActionType::CRAFTING_RECIPE_OPTIONAL => { Box::new(CraftRecipeOptionalStackRequestAction::read(stream)) as Box<dyn ItemStackRequestAction> }
            ItemStackRequestActionType::CRAFTING_GRINDSTONE => { Box::new(GrindstoneStackRequestAction::read(stream)) as Box<dyn ItemStackRequestAction> }
            ItemStackRequestActionType::CRAFTING_LOOM => { Box::new(LoomStackRequestAction::read(stream)) as Box<dyn ItemStackRequestAction> }
            ItemStackRequestActionType::CRAFTING_NON_IMPLEMENTED_DEPRECATED_ASK_TY_LAING => { Box::new(DeprecatedCraftingNonImplementedStackRequestAction::read(stream)) as Box<dyn ItemStackRequestAction> }
            ItemStackRequestActionType::CRAFTING_RESULTS_DEPRECATED_ASK_TY_LAING => { Box::new(DeprecatedCraftingResultsStackRequestAction::read(stream)) as Box<dyn ItemStackRequestAction> }
            _ => { panic!("Unhandled item stack request action type {}", type_id) },
        }
    }

    pub fn read(stream: &mut Stream) -> ItemStackRequestEntry {
        let request_id = PacketSerializer::read_item_stack_request_id(stream);
        let mut actions = Vec::new();
        let mut len = stream.get_var_u32();
        for _ in 0..len {
            let type_id = stream.get_byte();
            actions.push(Self::read_action(stream, type_id));
        }
        let mut filter_strings = Vec::new();
        len = stream.get_var_u32();
        for _ in 0..len {
            filter_strings.push(PacketSerializer::get_string(stream));
        }
        let filter_string_cause = stream.get_i32_le();

        ItemStackRequestEntry{ request_id, actions, filter_strings, filter_string_cause }
    }

    pub fn write(&mut self, stream: &mut Stream) {
        PacketSerializer::write_item_stack_request_id(stream, self.request_id);
        stream.put_var_u32(self.actions.len() as u32);
        for action in self.actions.iter_mut() {
            stream.put_byte(action.get_type_id());
            action.write(stream);
        }
        stream.put_var_u32(self.filter_strings.len() as u32);
        for filter_string in &self.filter_strings {
            PacketSerializer::put_string(stream, filter_string.clone());
        }
        stream.put_i32_le(self.filter_string_cause);
    }
}