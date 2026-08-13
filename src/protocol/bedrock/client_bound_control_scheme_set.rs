use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct ClientBoundControlSchemeSet {
    pub scheme: u8, //see types/control_scheme.rs
}

impl Packet for ClientBoundControlSchemeSet {
    fn id(&self) -> u16 {
        BedrockPacketType::IDClientBoundControlSchemeSet.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_u8(self.scheme);
    }

    fn decode(stream: &mut Reader) -> ClientBoundControlSchemeSet {
        let scheme = stream.get_u8();

        ClientBoundControlSchemeSet { scheme }
    }
}
