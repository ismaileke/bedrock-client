use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use std::any::Any;

#[derive(serde::Serialize, Debug)]
pub struct SetPlayerInventoryOptions {
    pub left_tab: i32,  //see types/inventory/inventory_left_tab.rs
    pub right_tab: i32, //see types/inventory/inventory_right_tab.rs
    pub filtering: bool,
    pub inventory_layout: i32, //see types/inventory/inventory_layout.rs
    pub crafting_layout: i32,
}

impl Packet for SetPlayerInventoryOptions {
    fn id(&self) -> u16 {
        BedrockPacketType::IDSetPlayerInventoryOptions.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_var_i32(self.left_tab);
        stream.put_var_i32(self.right_tab);
        stream.put_bool(self.filtering);
        stream.put_var_i32(self.inventory_layout);
        stream.put_var_i32(self.crafting_layout);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> SetPlayerInventoryOptions {
        let left_tab = stream.get_var_i32();
        let right_tab = stream.get_var_i32();
        let filtering = stream.get_bool();
        let inventory_layout = stream.get_var_i32();
        let crafting_layout = stream.get_var_i32();

        SetPlayerInventoryOptions {
            left_tab,
            right_tab,
            filtering,
            inventory_layout,
            crafting_layout,
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String { serde_json::to_string(self).unwrap() }
}
