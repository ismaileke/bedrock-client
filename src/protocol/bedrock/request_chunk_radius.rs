use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct RequestChunkRadius {
    pub radius: i32,
    pub max_radius: u8,
}

impl Packet for RequestChunkRadius {
    fn id(&self) -> u16 {
        BedrockPacketType::IDRequestChunkRadius.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_var_i32(self.radius);
        stream.put_u8(self.max_radius);
    }

    fn decode(stream: &mut Reader) -> RequestChunkRadius {
        let radius = stream.get_var_i32();
        let max_radius = stream.get_u8();

        RequestChunkRadius { radius, max_radius }
    }
}
