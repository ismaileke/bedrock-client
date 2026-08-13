use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct SetTime {
    pub time: i32,
}

impl Packet for SetTime {
    fn id(&self) -> u16 {
        BedrockPacketType::IDSetTime.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_var_i32(self.time);
    }

    fn decode(stream: &mut Reader) -> SetTime {
        let time = stream.get_var_i32();

        SetTime { time }
    }
}
