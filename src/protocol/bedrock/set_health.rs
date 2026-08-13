use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct SetHealth {
    pub health: i32,
}

impl Packet for SetHealth {
    fn id(&self) -> u16 {
        BedrockPacketType::IDSetHealth.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_var_i32(self.health);
    }

    fn decode(stream: &mut Reader) -> SetHealth {
        let health = stream.get_var_i32();

        SetHealth { health }
    }
}
