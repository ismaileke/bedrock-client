use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;

pub struct SetHud {
    pub hud_elements: Vec<i32>, //see types/hud/hud_element.rs
    pub visibility: i32 // see types/hud/hud_visibility.rs
}

pub fn new(hud_elements: Vec<i32>, visibility: i32) -> SetHud {
    SetHud { hud_elements, visibility }
}

impl Packet for SetHud {
    fn id(&self) -> u16 {
        BedrockPacketType::IDSetHud.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        stream.put_unsigned_var_int(self.hud_elements.len() as u32);
        for hud_element in self.hud_elements.iter() {
            stream.put_var_int(*hud_element);
        }
        stream.put_var_int(self.visibility);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> SetHud {
        let mut stream = Stream::new(bytes, 0);

        let count = stream.get_unsigned_var_int() as usize;
        let mut hud_elements = Vec::new();
        for _ in 0..count {
            hud_elements.push(stream.get_var_int());
        }
        let visibility = stream.get_var_int();

        SetHud { hud_elements, visibility }
    }

    fn debug(&self) {
        println!("Hud Elements: {:?}", self.hud_elements);
        println!("Hud Visibility: {}", self.visibility);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
