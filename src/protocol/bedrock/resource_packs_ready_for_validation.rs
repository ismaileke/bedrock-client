use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct ResourcePacksReadyForValidation {}

impl Packet for ResourcePacksReadyForValidation {
    fn id(&self) -> u16 {
        BedrockPacketType::IDResourcePacksReadyForValidation.get_u8()
    }

    fn encode(&mut self, _stream: &mut Writer) {
        // NO PAYLOAD
    }

    fn decode(_stream: &mut Reader) -> ResourcePacksReadyForValidation {
        // No Payload
        ResourcePacksReadyForValidation {}
    }
}
