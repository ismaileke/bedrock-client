use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::inventory::full_container_name::FullContainerName;
use crate::protocol::bedrock::types::inventory::item_stack_wrapper::ItemStackWrapper;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct InventorySlot {
    pub window_id: u32,
    pub inventory_slot: u32,
    pub container_name: Option<FullContainerName>,
    pub storage: Option<ItemStackWrapper>,
    pub item: ItemStackWrapper,
}

impl Packet for InventorySlot {
    fn id(&self) -> u16 {
        BedrockPacketType::IDInventorySlot.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_var_u32(self.window_id);
        stream.put_var_u32(self.inventory_slot);
        PacketSerializer::write_optional(stream, &self.container_name, |s, v| v.write(s));
        PacketSerializer::write_optional(stream, &self.storage, |s, v| PacketSerializer::put_network_item_stack_descriptor(s, v.clone()));
        PacketSerializer::put_network_item_stack_descriptor(stream, self.item.clone());
    }

    fn decode(stream: &mut Reader) -> InventorySlot {
        let window_id = stream.get_var_u32();
        let inventory_slot = stream.get_var_u32();
        let container_name = PacketSerializer::read_optional(stream, |s| FullContainerName::read(s));
        let storage = PacketSerializer::read_optional(stream, |s| PacketSerializer::get_network_item_stack_descriptor(s));
        let item = PacketSerializer::get_network_item_stack_descriptor(stream);

        InventorySlot {
            window_id,
            inventory_slot,
            container_name,
            storage,
            item,
        }
    }
}
