use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;

pub struct SetPlayerInventoryOptions {
    pub left_tab: i32, //see types/inventory/inventory_left_tab.rs
    pub right_tab: i32, //see types/inventory/inventory_right_tab.rs
    pub filtering: bool,
    pub inventory_layout: i32, //see types/inventory/inventory_layout.rs
    pub crafting_layout: i32
}

pub fn new(left_tab: i32, right_tab: i32, filtering: bool, inventory_layout: i32, crafting_layout: i32) -> SetPlayerInventoryOptions {
    SetPlayerInventoryOptions { left_tab, right_tab, filtering, inventory_layout, crafting_layout }
}

impl Packet for SetPlayerInventoryOptions {
    fn id(&self) -> u16 {
        BedrockPacketType::IDSetPlayerInventoryOptions.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        stream.put_var_int(self.left_tab);
        stream.put_var_int(self.right_tab);
        stream.put_bool(self.filtering);
        stream.put_var_int(self.inventory_layout);
        stream.put_var_int(self.crafting_layout);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> SetPlayerInventoryOptions {
        let mut stream = Stream::new(bytes, 0);

        let left_tab = stream.get_var_int();
        let right_tab = stream.get_var_int();
        let filtering = stream.get_bool();
        let inventory_layout = stream.get_var_int();
        let crafting_layout = stream.get_var_int();

        SetPlayerInventoryOptions { left_tab, right_tab, filtering, inventory_layout, crafting_layout }
    }

    fn debug(&self) {
        println!("Left Tab: {}", self.left_tab);
        println!("Right Tab: {}", self.right_tab);
        println!("Filtering: {}", self.filtering);
        println!("Inventory Layout: {}", self.inventory_layout);
        println!("Crafting Layout: {}", self.crafting_layout);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
