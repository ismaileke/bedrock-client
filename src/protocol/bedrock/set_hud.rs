use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct SetHud {
    pub hud_elements: Vec<i32>, //see types/hud/hud_element.rs
    pub visibility: i32,        // see types/hud/hud_visibility.rs
}

impl Packet for SetHud {
    fn id(&self) -> u16 {
        BedrockPacketType::IDSetHud.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_var_u32(self.hud_elements.len() as u32);
        for hud_element in self.hud_elements.iter() {
            stream.put_var_i32(*hud_element);
        }
        stream.put_var_i32(self.visibility);
    }

    fn decode(stream: &mut Reader) -> SetHud {
        let count = stream.get_var_u32() as usize;
        let mut hud_elements = Vec::new();
        for _ in 0..count {
            hud_elements.push(stream.get_var_i32());
        }
        let visibility = stream.get_var_i32();

        SetHud { hud_elements, visibility }
    }
}
