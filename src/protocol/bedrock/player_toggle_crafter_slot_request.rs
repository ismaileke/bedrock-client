use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;

pub struct PlayerToggleCrafterSlotRequest {
    pub block_position: Vec<u32>,
    pub slot: u8,
    pub disabled: bool
}

pub fn new(block_position: Vec<u32>, slot: u8, disabled: bool) -> PlayerToggleCrafterSlotRequest {
    PlayerToggleCrafterSlotRequest { block_position, slot, disabled }
}

impl Packet for PlayerToggleCrafterSlotRequest {
    fn id(&self) -> u16 {
        BedrockPacketType::IDPlayerToggleCrafterSlotRequest.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        stream.put_l_int(self.block_position[0]);
        stream.put_l_int(self.block_position[1]);
        stream.put_l_int(self.block_position[2]);
        stream.put_byte(self.slot);
        stream.put_bool(self.disabled);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> PlayerToggleCrafterSlotRequest {
        let mut stream = Stream::new(bytes, 0);

        let x = stream.get_l_int();
        let y = stream.get_l_int();
        let z = stream.get_l_int();
        let slot = stream.get_byte();
        let disabled = stream.get_bool();
        let block_position = vec![x, y, z];

        PlayerToggleCrafterSlotRequest { block_position, slot, disabled }
    }

    fn debug(&self) {
        println!("Block Position: {:?}", self.block_position);
        println!("Slot: {}", self.slot);
        println!("Disabled: {}", self.disabled);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
