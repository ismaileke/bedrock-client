use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct ClientCacheStatus {
    pub enabled: bool
}

impl Packet for ClientCacheStatus {
    fn id(&self) -> u16 {
        BedrockPacketType::IDClientCacheStatus.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_bool(self.enabled);
    }

    fn decode(stream: &mut Reader) -> ClientCacheStatus {
        let enabled = stream.get_bool();
        ClientCacheStatus { enabled }
    }
}
