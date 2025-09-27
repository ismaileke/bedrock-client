use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::inventory::full_container_name::FullContainerName;
use crate::protocol::bedrock::types::inventory::item_stack_wrapper::ItemStackWrapper;

pub struct InventorySlot {
    pub window_id: u32,
    pub inventory_slot: u32,
    pub container_name: FullContainerName,
    pub storage: ItemStackWrapper,
    pub item: ItemStackWrapper
}

pub fn new(window_id: u32, inventory_slot: u32, container_name: FullContainerName, storage: ItemStackWrapper, item: ItemStackWrapper) -> InventorySlot {
    InventorySlot { window_id, inventory_slot, container_name, storage, item }
}

impl Packet for InventorySlot {
    fn id(&self) -> u16 {
        BedrockPacketType::IDInventorySlot.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        stream.put_unsigned_var_int(self.window_id);
        stream.put_unsigned_var_int(self.inventory_slot);
        self.container_name.write(&mut stream);
        PacketSerializer::put_item_stack_wrapper(&mut stream, self.storage.clone());
        PacketSerializer::put_item_stack_wrapper(&mut stream, self.item.clone());

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> InventorySlot {
        let mut stream = Stream::new(bytes, 0);

        let window_id = stream.get_unsigned_var_int();
        let inventory_slot = stream.get_unsigned_var_int();
        let container_name = FullContainerName::read(&mut stream);
        let storage = PacketSerializer::get_item_stack_wrapper(&mut stream);
        let item = PacketSerializer::get_item_stack_wrapper(&mut stream);

        InventorySlot { window_id, inventory_slot, container_name, storage, item }
    }

    fn debug(&self) {
        println!("Window ID: {}", self.window_id);
        println!("Inventory Slot: {}", self.inventory_slot);
        println!("Container Name: {:?}", self.container_name);
        println!("Storage: {:?}", self.storage);
        println!("Item: {:?}", self.item);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
