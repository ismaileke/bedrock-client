use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct PlayerHotbar {
    pub selected_hotbar_slot: u32,
    pub window_id: u8, //see types/container_ids
    pub select_hotbar_slot: bool,
}

impl Packet for PlayerHotbar {
    fn id(&self) -> u16 {
        BedrockPacketType::IDPlayerHotbar.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_var_u32(self.selected_hotbar_slot);
        stream.put_u8(self.window_id);
        stream.put_bool(self.select_hotbar_slot);
    }

    fn decode(stream: &mut Reader) -> PlayerHotbar {
        let selected_hotbar_slot = stream.get_var_u32();
        let window_id = stream.get_u8();
        let select_hotbar_slot = stream.get_bool();

        PlayerHotbar {
            selected_hotbar_slot,
            window_id,
            select_hotbar_slot,
        }
    }
}
