use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::chunk_cache_blob::ChunkCacheBlob;

pub struct ClientCacheMissResponse {
    pub blobs: Vec<ChunkCacheBlob>
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
        stream.put_unsigned_var_int(self.id() as u32);

        stream.put_unsigned_var_int(self.blobs.len() as u32);
        for blob in self.blobs.iter() {
            stream.put_l_long(blob.get_hash());
            PacketSerializer::put_string(&mut stream, blob.get_payload());
        }

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> ClientCacheMissResponse {
        let mut stream = Stream::new(bytes, 0);

        let blobs_count = stream.get_unsigned_var_int() as usize;
        let mut blobs = Vec::new();
        for _ in 0..blobs_count {
            let hash = stream.get_l_long();
            let payload = PacketSerializer::get_string(&mut stream);
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
}
