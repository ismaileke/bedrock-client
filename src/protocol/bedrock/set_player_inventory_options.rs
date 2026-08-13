use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::{Reader, Writer};

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
        BedrockPacketType::IDSetPlayerInventoryOptions.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_var_i32(self.left_tab);
        stream.put_var_i32(self.right_tab);
        stream.put_bool(self.filtering);
        stream.put_var_i32(self.inventory_layout);
        stream.put_var_i32(self.crafting_layout);
    }

    fn decode(stream: &mut Reader) -> SetPlayerInventoryOptions {
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
}
