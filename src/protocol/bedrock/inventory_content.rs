use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::inventory::full_container_name::FullContainerName;
use crate::protocol::bedrock::types::inventory::item_stack_wrapper::ItemStackWrapper;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct InventoryContent {
    pub window_id: u32,
    pub items: Vec<ItemStackWrapper>,
    pub container_name: FullContainerName,
    pub storage: ItemStackWrapper,
}

impl Packet for InventoryContent {
    fn id(&self) -> u16 {
        BedrockPacketType::IDInventoryContent.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_var_u32(self.window_id);
        stream.put_var_u32(self.items.len() as u32);
        for item in &self.items {
            PacketSerializer::put_network_item_stack_descriptor(stream, item.clone());
        }
        self.container_name.write(stream);
        PacketSerializer::put_network_item_stack_descriptor(stream, self.storage.clone());
    }

    fn decode(stream: &mut Reader) -> InventoryContent {
        let window_id = stream.get_var_u32();
        let items_count = stream.get_var_u32();
        let mut items = Vec::new();
        for _ in 0..items_count {
            items.push(PacketSerializer::get_network_item_stack_descriptor(stream));
        }
        let container_name = FullContainerName::read(stream);
        let storage = PacketSerializer::get_network_item_stack_descriptor(stream);

        InventoryContent { window_id, items, container_name, storage }
    }
}
