use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct SetDefaultGameType {
    pub game_mode: i32,
}

impl Packet for SetDefaultGameType {
    fn id(&self) -> u16 {
        BedrockPacketType::IDSetDefaultGameType.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_var_i32(self.game_mode);
    }

    fn decode(stream: &mut Reader) -> SetDefaultGameType {
        let game_mode = stream.get_var_i32();

        SetDefaultGameType { game_mode }
    }
}
