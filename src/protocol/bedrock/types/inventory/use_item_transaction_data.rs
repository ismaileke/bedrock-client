use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::inventory::item_stack_wrapper::ItemStackWrapper;
use crate::protocol::bedrock::types::inventory::network_inventory_action::NetworkInventoryAction;
use binary_utils::binary::{Reader, Writer};
use crate::protocol::bedrock::types::inventory::item_stack::ItemStack;

#[derive(serde::Serialize, Debug, Clone)]
pub struct UseItemTransactionData {
    pub actions: Vec<NetworkInventoryAction>,
    pub action_type: i32,
    pub trigger_type: u8, //see types/inventory/see trigger_type.rs
    pub block_position: Vec<i32>,
    pub face: u8,
    pub hotbar_slot: i32,
    pub item_in_hand: ItemStackWrapper,
    pub player_position: Vec<f32>,
    pub click_position: Vec<f32>,
    pub block_runtime_id: u32,
    pub client_interact_prediction: u8, //see types/inventory/predicted_result.rs
    pub client_cooldown_state: u8
}

impl UseItemTransactionData {
    pub const ACTION_CLICK_BLOCK: i32 = 0;
    pub const ACTION_CLICK_AIR: i32 = 1;
    pub const ACTION_BREAK_BLOCK: i32 = 2;
    pub const ACTION_USE_AS_ATTACK: i32 = 3;

    pub fn new(
        actions: Vec<NetworkInventoryAction>,
        action_type: i32,
        trigger_type: u8,
        block_position: Vec<i32>,
        face: u8,
        hotbar_slot: i32,
        item_in_hand: ItemStackWrapper,
        player_position: Vec<f32>,
        click_position: Vec<f32>,
        block_runtime_id: u32,
        client_interact_prediction: u8,
        client_cooldown_state: u8
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
            client_cooldown_state,
        }
    }

    pub fn null() -> UseItemTransactionData {
        UseItemTransactionData {
            actions: vec![],
            action_type: 0,
            trigger_type: 0,
            block_position: vec![],
            face: 0,
            hotbar_slot: 0,
            item_in_hand: ItemStackWrapper {
                stack_id: 0,
                item_stack: ItemStack::null(),
            },
            player_position: vec![],
            click_position: vec![],
            block_runtime_id: 0,
            client_interact_prediction: 0,
            client_cooldown_state: 0,
        }
    }

    pub fn get_actions(&self) -> &Vec<NetworkInventoryAction> {
        self.actions.as_ref()
    }

    pub fn get_actions_mut(&mut self) -> &mut Vec<NetworkInventoryAction> {
        self.actions.as_mut()
    }

    pub fn decode_data(&mut self, stream: &mut Reader) {
        self.action_type = stream.get_var_i32();
        self.trigger_type = stream.get_u8();
        self.block_position = PacketSerializer::get_block_pos(stream);
        self.face = stream.get_u8();
        self.hotbar_slot = stream.get_var_i32();
        self.item_in_hand = PacketSerializer::get_network_item_stack_descriptor(stream);
        self.player_position = PacketSerializer::get_vector3(stream);
        self.click_position = PacketSerializer::get_vector3(stream);
        self.block_runtime_id = stream.get_var_u32();
        self.client_interact_prediction = stream.get_u8();
        self.client_cooldown_state = stream.get_u8();
    }

    pub fn encode_data(&self, stream: &mut Writer) {
        stream.put_var_i32(self.action_type);
        stream.put_u8(self.trigger_type);
        PacketSerializer::put_block_pos(stream, &self.block_position);
        stream.put_u8(self.face);
        stream.put_var_i32(self.hotbar_slot);
        PacketSerializer::put_network_item_stack_descriptor(stream, &self.item_in_hand);
        PacketSerializer::put_vector3(stream, &self.player_position);
        PacketSerializer::put_vector3(stream, &self.click_position);
        stream.put_var_u32(self.block_runtime_id);
        stream.put_u8(self.client_interact_prediction);
        stream.put_u8(self.client_cooldown_state);
    }

    /// PlayerAuthInput'a gömülü ItemUseTransaction ile InventoryTransaction
    /// paketindeki gövde AYNI: eylem dizisi + veri. Ayrı bir "item interaction"
    /// biçimi yok; eskiden buraya iki adet dummy bool yazılıyordu ve sunucu
    /// paketi çözemiyordu.
    pub fn decode_with_actions(&mut self, stream: &mut Reader) {
        let action_count = stream.get_var_u32();
        for _ in 0..action_count {
            self.actions.push(NetworkInventoryAction::read(stream));
        }
        self.decode_data(stream);
    }

    pub fn encode_with_actions(&self, stream: &mut Writer) {
        stream.put_var_u32(self.actions.len() as u32);
        for action in self.actions.iter() {
            action.write(stream);
        }
        self.encode_data(stream);
    }
}
