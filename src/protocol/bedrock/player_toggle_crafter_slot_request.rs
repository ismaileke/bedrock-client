use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct PlayerToggleCrafterSlotRequest {
    pub block_position: Vec<i32>,
    pub slot: u8,
    pub disabled: bool,
}

impl Packet for PlayerToggleCrafterSlotRequest {
    fn id(&self) -> u16 {
        BedrockPacketType::IDPlayerToggleCrafterSlotRequest.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_i32_le(self.block_position[0]);
        stream.put_i32_le(self.block_position[1]);
        stream.put_i32_le(self.block_position[2]);
        stream.put_u8(self.slot);
        stream.put_bool(self.disabled);
    }

    fn decode(stream: &mut Reader) -> PlayerToggleCrafterSlotRequest {
        let x = stream.get_i32_le();
        let y = stream.get_i32_le();
        let z = stream.get_i32_le();
        let slot = stream.get_u8();
        let disabled = stream.get_bool();
        let block_position = vec![x, y, z];

        PlayerToggleCrafterSlotRequest { block_position, slot, disabled }
    }
}
