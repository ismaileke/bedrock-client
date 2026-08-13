use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::inventory::item_stack_wrapper::ItemStackWrapper;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct MobEquipment {
    pub actor_runtime_id: u64,
    pub item: ItemStackWrapper,
    pub inventory_slot: u8,
    pub hotbar_slot: u8,
    pub window_id: u8,
}

impl Packet for MobEquipment {
    fn id(&self) -> u16 {
        BedrockPacketType::IDMobEquipment.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_actor_runtime_id(stream, self.actor_runtime_id);
        PacketSerializer::put_network_item_stack_descriptor(stream, self.item.clone());
        stream.put_u8(self.inventory_slot);
        stream.put_u8(self.hotbar_slot);
        stream.put_u8(self.window_id);
    }

    fn decode(stream: &mut Reader) -> MobEquipment {
        let actor_runtime_id = PacketSerializer::get_actor_runtime_id(stream);
        let item = PacketSerializer::get_network_item_stack_descriptor(stream);
        let inventory_slot = stream.get_u8();
        let hotbar_slot = stream.get_u8();
        let window_id = stream.get_u8();

        MobEquipment { actor_runtime_id, item, inventory_slot, hotbar_slot, window_id }
    }
}
