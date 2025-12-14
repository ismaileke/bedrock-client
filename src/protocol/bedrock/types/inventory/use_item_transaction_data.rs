use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::inventory::item_stack_wrapper::ItemStackWrapper;
use crate::protocol::bedrock::types::inventory::network_inventory_action::NetworkInventoryAction;
use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug, Clone)]
pub struct UseItemTransactionData {
    actions: Vec<NetworkInventoryAction>,
    action_type: u32,
    trigger_type: u32, //see types/inventory/see trigger_type.rs
    block_position: Vec<i32>,
    face: i32,
    hotbar_slot: i32,
    item_in_hand: ItemStackWrapper,
    player_position: Vec<f32>,
    click_position: Vec<f32>,
    block_runtime_id: u32,
    client_interact_prediction: u32, //see types/inventory/predicted_result.rs
}

impl UseItemTransactionData {
    pub const ACTION_CLICK_BLOCK: u32 = 0;
    pub const ACTION_CLICK_AIR: u32 = 1;
    pub const ACTION_BREAK_BLOCK: u32 = 2;
    pub const ACTION_USE_AS_ATTACK: u32 = 3;

    pub fn new(
        actions: Vec<NetworkInventoryAction>,
        action_type: u32,
        trigger_type: u32,
        block_position: Vec<i32>,
        face: i32,
        hotbar_slot: i32,
        item_in_hand: ItemStackWrapper,
        player_position: Vec<f32>,
        click_position: Vec<f32>,
        block_runtime_id: u32,
        client_interact_prediction: u32,
    ) -> UseItemTransactionData {
        UseItemTransactionData {
            actions,
            action_type,
            trigger_type,
            block_position,
            face,
            hotbar_slot,
            item_in_hand,
            player_position,
            click_position,
            block_runtime_id,
            client_interact_prediction,
        }
    }

    pub fn get_actions(&self) -> &Vec<NetworkInventoryAction> {
        self.actions.as_ref()
    }

    pub fn get_actions_mut(&mut self) -> &mut Vec<NetworkInventoryAction> {
        self.actions.as_mut()
    }

    pub fn decode_data(&mut self, stream: &mut Stream) {
        self.action_type = stream.get_var_u32();
        self.trigger_type = stream.get_var_u32();
        self.block_position = PacketSerializer::get_block_pos(stream);
        self.face = stream.get_var_i32();
        self.hotbar_slot = stream.get_var_i32();
        self.item_in_hand = PacketSerializer::get_item_stack_wrapper(stream);
        self.player_position = PacketSerializer::get_vector3(stream);
        self.click_position = PacketSerializer::get_vector3(stream);
        self.block_runtime_id = stream.get_var_u32();
        self.client_interact_prediction = stream.get_var_u32();
    }

    pub fn encode_data(&self, stream: &mut Stream) {
        stream.put_var_u32(self.action_type);
        stream.put_var_u32(self.trigger_type);
        PacketSerializer::put_block_pos(stream, self.block_position.clone());
        stream.put_var_i32(self.face);
        stream.put_var_i32(self.hotbar_slot);
        PacketSerializer::put_item_stack_wrapper(stream, self.item_in_hand.clone());
        PacketSerializer::put_vector3(stream, self.player_position.clone());
        PacketSerializer::put_vector3(stream, self.click_position.clone());
        stream.put_var_u32(self.block_runtime_id);
        stream.put_var_u32(self.client_interact_prediction);
    }
}
