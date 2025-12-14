use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use std::any::Any;

#[derive(serde::Serialize, Debug)]
pub struct PlayerHotbar {
    pub selected_hotbar_slot: u32,
    pub window_id: u8, //see types/container_ids
    pub select_hotbar_slot: bool,
}

pub fn new(selected_hotbar_slot: u32, window_id: u8, select_hotbar_slot: bool) -> PlayerHotbar {
    PlayerHotbar {
        selected_hotbar_slot,
        window_id,
        select_hotbar_slot,
    }
}

impl Packet for PlayerHotbar {
    fn id(&self) -> u16 {
        BedrockPacketType::IDPlayerHotbar.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_var_u32(self.selected_hotbar_slot);
        stream.put_byte(self.window_id);
        stream.put_bool(self.select_hotbar_slot);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> PlayerHotbar {
        let selected_hotbar_slot = stream.get_var_u32();
        let window_id = stream.get_byte();
        let select_hotbar_slot = stream.get_bool();

        PlayerHotbar {
            selected_hotbar_slot,
            window_id,
            select_hotbar_slot,
        }
    }

    fn debug(&self) {
        println!("Selected Hotbar Slot: {}", self.selected_hotbar_slot);
        println!("Window ID: {}", self.window_id);
        println!("Select Hotbar Slot: {}", self.select_hotbar_slot);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
