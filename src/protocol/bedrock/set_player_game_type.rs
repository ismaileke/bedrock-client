use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct SetPlayerGameType {
    pub game_mode: i32,
}

impl Packet for SetPlayerGameType {
    fn id(&self) -> u16 {
        BedrockPacketType::IDSetPlayerGameType.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_var_i32(self.game_mode);
    }

    fn decode(stream: &mut Reader) -> SetPlayerGameType {
        let game_mode = stream.get_var_i32();

        SetPlayerGameType { game_mode }
    }
}
