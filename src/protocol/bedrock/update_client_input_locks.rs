use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct UpdateClientInputLocks {
    pub flags: u32,
}

impl Packet for UpdateClientInputLocks {
    fn id(&self) -> u16 {
        BedrockPacketType::IDUpdateClientInputLocks.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_var_u32(self.flags);
    }

    fn decode(stream: &mut Reader) -> UpdateClientInputLocks {
        let flags = stream.get_var_u32();

        UpdateClientInputLocks { flags }
    }
}
