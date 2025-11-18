use binary_utils::binary::Stream;
use crate::protocol::bedrock::inventory_transaction::InventoryTransaction;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::inventory::item_stack_wrapper::ItemStackWrapper;
use crate::protocol::bedrock::types::inventory::network_inventory_action::NetworkInventoryAction;
use crate::protocol::bedrock::types::inventory::transaction_data::TransactionData;

#[derive(Debug)]
pub struct ReleaseItemTransactionData {
    actions: Vec<NetworkInventoryAction>,
    action_type: u32,
    hotbar_slot: i32,
    item_in_hand: ItemStackWrapper,
    head_position: Vec<f32>
}

impl ReleaseItemTransactionData {
    pub const ACTION_RELEASE: u32 = 0; //bow shoot
    pub const ACTION_CONSUME: u32 = 1; //eat food, drink potion

    pub fn new(
        actions: Vec<NetworkInventoryAction>,
        action_type: u32,
        hotbar_slot: i32,
        item_in_hand: ItemStackWrapper,
        head_position: Vec<f32>
    ) -> ReleaseItemTransactionData {
        ReleaseItemTransactionData{ actions, action_type, hotbar_slot, item_in_hand, head_position }
    }
}

impl TransactionData for ReleaseItemTransactionData {
    fn get_type_id(&self) -> u32 {
        InventoryTransaction::TYPE_RELEASE_ITEM
    }

    fn get_actions(&self) -> &Vec<NetworkInventoryAction> {
        self.actions.as_ref()
    }

    fn get_actions_mut(&mut self) -> &mut Vec<NetworkInventoryAction> {
        self.actions.as_mut()
    }

    fn decode_data(&mut self, stream: &mut Stream) {
        self.action_type = stream.get_var_u32();
        self.hotbar_slot = stream.get_var_i32();
        self.item_in_hand = PacketSerializer::get_item_stack_wrapper(stream);
        self.head_position = PacketSerializer::get_vector3(stream);
    }

    fn encode_data(&self, stream: &mut Stream) {
        stream.put_var_u32(self.action_type);
        stream.put_var_i32(self.hotbar_slot);
        PacketSerializer::put_item_stack_wrapper(stream, self.item_in_hand.clone());
        PacketSerializer::put_vector3(stream, self.head_position.clone());
    }
}