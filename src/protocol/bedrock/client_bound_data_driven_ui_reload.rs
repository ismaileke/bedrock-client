use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct ClientBoundDataDrivenUIReload {}

impl Packet for ClientBoundDataDrivenUIReload {
    fn id(&self) -> u16 {
        BedrockPacketType::IDClientBoundDataDrivenUIReload.get_u8()
    }

    fn encode(&mut self, _stream: &mut Writer) {
        // NO PAYLOAD
    }

    fn decode(_stream: &mut Reader) -> ClientBoundDataDrivenUIReload {
        // NO PAYLOAD

        ClientBoundDataDrivenUIReload {}
    }
}
