use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct ServerStats {
    pub server_time: f32,
    pub network_time: f32,
}

impl Packet for ServerStats {
    fn id(&self) -> u16 {
        BedrockPacketType::IDServerStats.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_f32_le(self.server_time);
        stream.put_f32_le(self.network_time);
    }

    fn decode(stream: &mut Reader) -> ServerStats {
        let server_time = stream.get_f32_le();
        let network_time = stream.get_f32_le();

        ServerStats { server_time, network_time }
    }
}
