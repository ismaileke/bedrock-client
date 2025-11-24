use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::inventory::full_container_name::FullContainerName;
use crate::protocol::bedrock::types::inventory::item_stack_wrapper::ItemStackWrapper;

#[derive(serde::Serialize, Debug)]
pub struct InventoryContent {
    pub window_id: u32,
    pub items: Vec<ItemStackWrapper>,
    pub container_name: FullContainerName,
    pub storage: ItemStackWrapper
}

pub fn new(window_id: u32, items: Vec<ItemStackWrapper>, container_name: FullContainerName, storage: ItemStackWrapper) -> InventoryContent {
    InventoryContent { window_id, items, container_name, storage }
}

impl Packet for InventoryContent {
    fn id(&self) -> u16 {
        BedrockPacketType::IDInventoryContent.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_var_u32(self.window_id);
        stream.put_var_u32(self.items.len() as u32);
        for item in &self.items {
            PacketSerializer::put_item_stack_wrapper(&mut stream, item.clone());
        }
        self.container_name.write(&mut stream);
        PacketSerializer::put_item_stack_wrapper(&mut stream, self.storage.clone());

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> InventoryContent {
        let window_id = stream.get_var_u32();
        let items_count = stream.get_var_u32();
        let mut items = Vec::new();
        for _ in 0..items_count {
            items.push(PacketSerializer::get_item_stack_wrapper(stream));
        }
        let container_name = FullContainerName::read(stream);
        let storage = PacketSerializer::get_item_stack_wrapper(stream);

        InventoryContent { window_id, items, container_name, storage }
    }

    fn debug(&self) {
        println!("Window ID: {}", self.window_id);
        println!("Items: {:?}", self.items);
        println!("Container Name: {:?}", self.container_name);
        println!("Storage: {:?}", self.storage);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
