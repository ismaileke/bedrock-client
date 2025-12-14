use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::chunk_cache_blob::ChunkCacheBlob;
use binary_utils::binary::Stream;
use std::any::Any;

#[derive(serde::Serialize, Debug)]
pub struct ClientCacheMissResponse {
    pub blobs: Vec<ChunkCacheBlob>,
}

pub fn new(blobs: Vec<ChunkCacheBlob>) -> ClientCacheMissResponse {
    ClientCacheMissResponse { blobs }
}

impl Packet for ClientCacheMissResponse {
    fn id(&self) -> u16 {
        BedrockPacketType::IDClientCacheMissResponse.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_var_u32(self.blobs.len() as u32);
        for blob in self.blobs.iter() {
            stream.put_u64_le(blob.get_hash());
            PacketSerializer::put_string(&mut stream, blob.get_payload());
        }

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> ClientCacheMissResponse {
        let blobs_count = stream.get_var_u32() as usize;
        let mut blobs = Vec::new();
        for _ in 0..blobs_count {
            let hash = stream.get_u64_le();
            let payload = PacketSerializer::get_string(stream);
            blobs.push(ChunkCacheBlob::new(hash, payload));
        }

        ClientCacheMissResponse { blobs }
    }

    fn debug(&self) {
        println!("Blobs: {:?}", self.blobs);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
