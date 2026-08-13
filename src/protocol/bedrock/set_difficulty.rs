use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct SetDifficulty {
    pub difficulty: u32,
}

impl Packet for SetDifficulty {
    fn id(&self) -> u16 {
        BedrockPacketType::IDSetDifficulty.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_var_u32(self.difficulty);
    }

    fn decode(stream: &mut Reader) -> SetDifficulty {
        let difficulty = stream.get_var_u32();

        SetDifficulty { difficulty }
    }
}
