use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::chunk_cache_blob::ChunkCacheBlob;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct ClientCacheMissResponse {
    pub blobs: Vec<ChunkCacheBlob>,
}

impl Packet for ClientCacheMissResponse {
    fn id(&self) -> u16 {
        BedrockPacketType::IDClientCacheMissResponse.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_var_u32(self.blobs.len() as u32);
        for blob in self.blobs.iter() {
            stream.put_u64_le(blob.get_hash());
            PacketSerializer::put_string(stream, &blob.get_payload());
        }
    }

    fn decode(stream: &mut Reader) -> ClientCacheMissResponse {
        let blobs_count = stream.get_var_u32() as usize;
        let mut blobs = Vec::new();
        for _ in 0..blobs_count {
            let hash = stream.get_u64_le();
            let payload = PacketSerializer::get_string(stream);
            blobs.push(ChunkCacheBlob::new(hash, payload));
        }

        ClientCacheMissResponse { blobs }
    }
}
