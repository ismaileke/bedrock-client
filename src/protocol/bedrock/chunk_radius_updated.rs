use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct ChunkRadiusUpdated {
    pub radius: i32
}

impl Packet for ChunkRadiusUpdated {
    fn id(&self) -> u16 {
        BedrockPacketType::IDChunkRadiusUpdated.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_var_i32(self.radius);
    }

    fn decode(stream: &mut Reader) -> ChunkRadiusUpdated {
        let radius = stream.get_var_i32();

        ChunkRadiusUpdated { radius }
    }
}
