use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};
use log::error;

const MAX_SAVED_CHUNKS: u32 = 9216;

#[derive(serde::Serialize, Debug)]
pub struct NetworkChunkPublisherUpdate {
    pub block_pos: Vec<i32>,
    pub radius: u32,
    pub saved_chunks: Vec<Vec<i32>>,
}

impl Packet for NetworkChunkPublisherUpdate {
    fn id(&self) -> u16 {
        BedrockPacketType::IDNetworkChunkPublisherUpdate.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_block_pos(stream, &self.block_pos);

        stream.put_var_u32(self.radius);

        stream.put_u32_le(self.saved_chunks.len() as u32);
        for chunk in self.saved_chunks.iter() {
            stream.put_var_i32(chunk[0]);
            stream.put_var_i32(chunk[1]);
        }
    }

    fn decode(stream: &mut Reader) -> NetworkChunkPublisherUpdate {
        let block_pos = PacketSerializer::get_block_pos(stream);
        let radius = stream.get_var_u32();
        let count = stream.get_u32_le();
        if count > MAX_SAVED_CHUNKS {
            error!("Expected at most {} saved chunks, got {}",MAX_SAVED_CHUNKS, count)
        }

        let mut saved_chunks = vec![];
        for _ in 0..count {
            let chunk_x = stream.get_var_i32();
            let chunk_z = stream.get_var_i32();
            saved_chunks.push(vec![chunk_x, chunk_z]);
        }

        NetworkChunkPublisherUpdate {
            block_pos,
            radius,
            saved_chunks,
        }
    }
}
