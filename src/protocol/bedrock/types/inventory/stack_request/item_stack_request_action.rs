use std::fmt::Debug;
use binary_utils::binary::Stream;
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
use crate::protocol::bedrock::types::inventory::stack_request::item_stack_request_action_type::ItemStackRequestActionType;
use crate::protocol::bedrock::types::inventory::stack_request::lab_table_combine_input_stack_request_action::LabTableCombineInputStackRequestAction;
use crate::protocol::bedrock::types::inventory::stack_request::loom_stack_request_action::LoomStackRequestAction;
use crate::protocol::bedrock::types::inventory::stack_request::mine_block_stack_request_action::MineBlockStackRequestAction;
use crate::protocol::bedrock::types::inventory::stack_request::place_stack_request_action::PlaceStackRequestAction;
use crate::protocol::bedrock::types::inventory::stack_request::swap_stack_request_action::SwapStackRequestAction;
use crate::protocol::bedrock::types::inventory::stack_request::take_stack_request_action::TakeStackRequestAction;

#[derive(serde::Serialize, Debug)]
pub enum ItemStackRequestAction {
    Take(TakeStackRequestAction),
    Place(PlaceStackRequestAction),
    Swap(SwapStackRequestAction),
    Drop(DropStackRequestAction),
    Destroy(DestroyStackRequestAction),
    CraftingConsumeInput(CraftingConsumeInputStackRequestAction),
    LabTableCombine(LabTableCombineInputStackRequestAction),
    BeaconPayment(BeaconPaymentStackRequestAction),
    MineBlock(MineBlockStackRequestAction),
    CraftRecipe(CraftRecipeStackRequestAction),
    CraftRecipeAuto(CraftRecipeAutoStackRequestAction),
    CreativeCreate(CreativeCreateStackRequestAction),
    CraftRecipeOptional(CraftRecipeOptionalStackRequestAction),
    Grindstone(GrindstoneStackRequestAction),
    Loom(LoomStackRequestAction),
    DeprecatedCraftingNonImplemented(DeprecatedCraftingNonImplementedStackRequestAction),
    DeprecatedCraftingResults(DeprecatedCraftingResultsStackRequestAction)
}

impl ItemStackRequestAction {
    pub fn get_type_id(&self) -> u8 {
        match self {
            ItemStackRequestAction::Take(_)  => ItemStackRequestActionType::TAKE,
            ItemStackRequestAction::Place(_) => ItemStackRequestActionType::PLACE,
            ItemStackRequestAction::Swap(_)  => ItemStackRequestActionType::SWAP,
            ItemStackRequestAction::Drop(_)  => ItemStackRequestActionType::DROP,
            ItemStackRequestAction::Destroy(_) => ItemStackRequestActionType::DESTROY,
            ItemStackRequestAction::CraftingConsumeInput(_) => ItemStackRequestActionType::CRAFTING_CONSUME_INPUT,
            ItemStackRequestAction::LabTableCombine(_) => ItemStackRequestActionType::LAB_TABLE_COMBINE,
            ItemStackRequestAction::BeaconPayment(_) => ItemStackRequestActionType::BEACON_PAYMENT,
            ItemStackRequestAction::MineBlock(_) => ItemStackRequestActionType::MINE_BLOCK,
            ItemStackRequestAction::CraftRecipe(_) => ItemStackRequestActionType::CRAFTING_RECIPE,
            ItemStackRequestAction::CraftRecipeAuto(_) => ItemStackRequestActionType::CRAFTING_RECIPE_AUTO,
            ItemStackRequestAction::CreativeCreate(_) => ItemStackRequestActionType::CREATIVE_CREATE,
            ItemStackRequestAction::CraftRecipeOptional(_) => ItemStackRequestActionType::CRAFTING_RECIPE_OPTIONAL,
            ItemStackRequestAction::Grindstone(_) => ItemStackRequestActionType::CRAFTING_GRINDSTONE,
            ItemStackRequestAction::Loom(_) => ItemStackRequestActionType::CRAFTING_LOOM,
            ItemStackRequestAction::DeprecatedCraftingNonImplemented(_) => ItemStackRequestActionType::CRAFTING_NON_IMPLEMENTED_DEPRECATED_ASK_TY_LAING,
            ItemStackRequestAction::DeprecatedCraftingResults(_) => ItemStackRequestActionType::CRAFTING_RESULTS_DEPRECATED_ASK_TY_LAING
        }
    }

    pub fn write(&mut self, stream: &mut Stream) {
        match self {
            ItemStackRequestAction::Take(r)  => r.write(stream),
            ItemStackRequestAction::Place(r) => r.write(stream),
            ItemStackRequestAction::Swap(r)  => r.write(stream),
            ItemStackRequestAction::Drop(r)  => r.write(stream),
            ItemStackRequestAction::Destroy(r) => r.write(stream),
            ItemStackRequestAction::CraftingConsumeInput(r) => r.write(stream),
            ItemStackRequestAction::LabTableCombine(r) => r.write(stream),
            ItemStackRequestAction::BeaconPayment(r) => r.write(stream),
            ItemStackRequestAction::MineBlock(r) => r.write(stream),
            ItemStackRequestAction::CraftRecipe(r) => r.write(stream),
            ItemStackRequestAction::CraftRecipeAuto(r) => r.write(stream),
            ItemStackRequestAction::CreativeCreate(r) => r.write(stream),
            ItemStackRequestAction::CraftRecipeOptional(r) => r.write(stream),
            ItemStackRequestAction::Grindstone(r) => r.write(stream),
            ItemStackRequestAction::Loom(r) => r.write(stream),
            ItemStackRequestAction::DeprecatedCraftingNonImplemented(r) => r.write(stream),
            ItemStackRequestAction::DeprecatedCraftingResults(r) => r.write(stream)
        }
    }
}
