use binary_utils::binary::Stream;
use crate::protocol::bedrock::inventory_transaction::InventoryTransaction;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::inventory::item_stack_wrapper::ItemStackWrapper;
use crate::protocol::bedrock::types::inventory::network_inventory_action::NetworkInventoryAction;
use crate::protocol::bedrock::types::inventory::transaction_data::TransactionData;

#[derive(Debug)]
pub struct UseItemOnEntityTransactionData {
    actions: Vec<NetworkInventoryAction>,
    actor_runtime_id: u64,
    action_type: u32,
    hotbar_slot: i32,
    item_in_hand: ItemStackWrapper,
    player_position: Vec<f32>,
    click_position: Vec<f32>
}

impl UseItemOnEntityTransactionData {
    pub const ACTION_INTERACT: u32 = 0;
    pub const ACTION_ATTACK: u32 = 1;
    pub const ACTION_ITEM_INTERACT: u32 = 2;

    pub fn new(
        actions: Vec<NetworkInventoryAction>,
        actor_runtime_id: u64,
        action_type: u32,
        hotbar_slot: i32,
        item_in_hand: ItemStackWrapper,
        player_position: Vec<f32>,
        click_position: Vec<f32>
    ) -> UseItemOnEntityTransactionData {
        UseItemOnEntityTransactionData{ actions, actor_runtime_id, action_type, hotbar_slot, item_in_hand, player_position, click_position }
    }
}

impl TransactionData for UseItemOnEntityTransactionData {
    fn get_type_id(&self) -> u32 {
        InventoryTransaction::TYPE_USE_ITEM_ON_ENTITY
    }

    fn get_actions(&self) -> &Vec<NetworkInventoryAction> {
        self.actions.as_ref()
    }

    fn get_actions_mut(&mut self) -> &mut Vec<NetworkInventoryAction> {
        self.actions.as_mut()
    }

    fn decode_data(&mut self, stream: &mut Stream) {
        self.actor_runtime_id = PacketSerializer::get_actor_runtime_id(stream);
        self.action_type = stream.get_unsigned_var_int();
        self.hotbar_slot = stream.get_var_int();
        self.item_in_hand = PacketSerializer::get_item_stack_wrapper(stream);
        self.player_position = PacketSerializer::get_vector3(stream);
        self.click_position = PacketSerializer::get_vector3(stream);
    }

    fn encode_data(&self, stream: &mut Stream) {
        PacketSerializer::put_actor_runtime_id(stream, self.actor_runtime_id);
        stream.put_unsigned_var_int(self.action_type);
        stream.put_var_int(self.hotbar_slot);
        PacketSerializer::put_item_stack_wrapper(stream, self.item_in_hand.clone());
        PacketSerializer::put_vector3(stream, self.player_position.clone());
        PacketSerializer::put_vector3(stream, self.click_position.clone());
    }
}