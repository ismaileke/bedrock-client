use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct TickingAreasLoadStatus {
    pub waiting_for_preload: bool,
}

impl Packet for TickingAreasLoadStatus {
    fn id(&self) -> u16 {
        BedrockPacketType::IDTickingAreasLoadStatus.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_bool(self.waiting_for_preload);
    }

    fn decode(stream: &mut Reader) -> TickingAreasLoadStatus {
        let waiting_for_preload = stream.get_bool();

        TickingAreasLoadStatus { waiting_for_preload }
    }
}
