use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct ServerBoundMatchmakingCancel {}

impl Packet for ServerBoundMatchmakingCancel {
    fn id(&self) -> u16 {
        BedrockPacketType::IDServerBoundMatchmakingCancel.get_u8()
    }

    fn encode(&mut self, _: &mut Writer) {
    }

    fn decode(_: &mut Reader) -> ServerBoundMatchmakingCancel {
        ServerBoundMatchmakingCancel { }
    }
}
