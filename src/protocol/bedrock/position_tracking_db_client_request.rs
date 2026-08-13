use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct PositionTrackingDBClientRequest {
    pub action: u8,
    pub tracking_id: i32,
}

impl PositionTrackingDBClientRequest {
    pub const ACTION_QUERY: u8 = 0;
}

impl Packet for PositionTrackingDBClientRequest {
    fn id(&self) -> u16 {
        BedrockPacketType::IDPositionTrackingDBClientRequest.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_u8(self.action);
        stream.put_var_i32(self.tracking_id);
    }

    fn decode(stream: &mut Reader) -> PositionTrackingDBClientRequest {
        let action = stream.get_u8();
        let tracking_id = stream.get_var_i32();

        PositionTrackingDBClientRequest { action, tracking_id }
    }
}
