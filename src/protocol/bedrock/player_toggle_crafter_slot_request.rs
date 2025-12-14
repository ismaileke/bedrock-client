use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use std::any::Any;

#[derive(serde::Serialize, Debug)]
pub struct PlayerToggleCrafterSlotRequest {
    pub block_position: Vec<i32>,
    pub slot: u8,
    pub disabled: bool,
}

pub fn new(block_position: Vec<i32>, slot: u8, disabled: bool) -> PlayerToggleCrafterSlotRequest {
    PlayerToggleCrafterSlotRequest {
        block_position,
        slot,
        disabled,
    }
}

impl Packet for PlayerToggleCrafterSlotRequest {
    fn id(&self) -> u16 {
        BedrockPacketType::IDPlayerToggleCrafterSlotRequest.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_i32_le(self.block_position[0]);
        stream.put_i32_le(self.block_position[1]);
        stream.put_i32_le(self.block_position[2]);
        stream.put_byte(self.slot);
        stream.put_bool(self.disabled);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> PlayerToggleCrafterSlotRequest {
        let x = stream.get_i32_le();
        let y = stream.get_i32_le();
        let z = stream.get_i32_le();
        let slot = stream.get_byte();
        let disabled = stream.get_bool();
        let block_position = vec![x, y, z];

        PlayerToggleCrafterSlotRequest {
            block_position,
            slot,
            disabled,
        }
    }

    fn debug(&self) {
        println!("Block Position: {:?}", self.block_position);
        println!("Slot: {}", self.slot);
        println!("Disabled: {}", self.disabled);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
