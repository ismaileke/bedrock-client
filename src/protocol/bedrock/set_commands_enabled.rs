use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct SetCommandsEnabled {
    pub enabled: bool,
}

impl Packet for SetCommandsEnabled {
    fn id(&self) -> u16 {
        BedrockPacketType::IDSetCommandsEnabled.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_bool(self.enabled);
    }

    fn decode(stream: &mut Reader) -> SetCommandsEnabled {
        let enabled = stream.get_bool();

        SetCommandsEnabled { enabled }
    }
}
