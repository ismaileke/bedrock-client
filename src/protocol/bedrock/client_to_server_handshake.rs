use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct ClientToServerHandshake {}

impl Packet for ClientToServerHandshake {
    fn id(&self) -> u16 {
        BedrockPacketType::IDClientToServerHandshake.get_u8()
    }

    fn encode(&mut self, _stream: &mut Writer) {}

    fn decode(_stream: &mut Reader) -> ClientToServerHandshake {
        ClientToServerHandshake {}
    }
}
