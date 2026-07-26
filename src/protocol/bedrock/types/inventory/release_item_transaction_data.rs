use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::inventory::item_stack_wrapper::ItemStackWrapper;
use crate::protocol::bedrock::types::inventory::network_inventory_action::NetworkInventoryAction;
use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct ReleaseItemTransactionData {
    actions: Vec<NetworkInventoryAction>,
    action_type: i32,
    hotbar_slot: i32,
    item_in_hand: ItemStackWrapper,
    head_position: Vec<f32>,
}

impl ReleaseItemTransactionData {
    pub const ACTION_RELEASE: i32 = 0; //bow shoot
    pub const ACTION_CONSUME: i32 = 1; //eat food, drink potion

    pub fn new(
        actions: Vec<NetworkInventoryAction>,
        action_type: i32,
        hotbar_slot: i32,
        item_in_hand: ItemStackWrapper,
        head_position: Vec<f32>,
    ) -> ReleaseItemTransactionData {
        ReleaseItemTransactionData {
            actions,
            action_type,
            hotbar_slot,
            item_in_hand,
            head_position,
        }
    }

    pub fn get_actions(&self) -> &Vec<NetworkInventoryAction> {
        self.actions.as_ref()
    }

    pub fn get_actions_mut(&mut self) -> &mut Vec<NetworkInventoryAction> {
        self.actions.as_mut()
    }

    pub fn decode_data(&mut self, stream: &mut Stream) {
        self.action_type = stream.get_var_i32();
        self.hotbar_slot = stream.get_var_i32();
        self.item_in_hand = PacketSerializer::get_network_item_stack_descriptor(stream);
        self.head_position = PacketSerializer::get_vector3(stream);
    }

    pub fn encode_data(&self, stream: &mut Stream) {
        stream.put_var_i32(self.action_type);
        stream.put_var_i32(self.hotbar_slot);
        PacketSerializer::put_network_item_stack_descriptor(stream, self.item_in_hand.clone());
        PacketSerializer::put_vector3(stream, self.head_position.clone());
    }
}
