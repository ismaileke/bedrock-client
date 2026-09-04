use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::{Reader, Writer};
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

#[derive(serde::Serialize, Debug)]
pub struct LevelChunk {
    pub chunk_x: i32,
    pub chunk_z: i32,
    pub dimension_id: i32,
    pub sub_chunk_count: u32,
    pub client_request_sub_chunk_limit: Option<i32>,
    pub cache_enabled: bool,
    pub used_blob_hashes: Vec<u64>,
    pub extra_payload: Vec<u8>,
}

impl LevelChunk {
    //this appears large enough for a world height of 1024 blocks - it may need to be increased in the future
    pub const MAX_BLOB_HASHES: u32 = 64;
}

impl Packet for LevelChunk {
    fn id(&self) -> u16 {
        BedrockPacketType::IDLevelChunk.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_var_i32(self.chunk_x);
        stream.put_var_i32(self.chunk_z);
        stream.put_var_i32(self.dimension_id);
        stream.put_var_u32(self.sub_chunk_count);
        PacketSerializer::write_optional(stream, &self.client_request_sub_chunk_limit, |s, v| s.put_var_i32(*v));
        stream.put_bool(self.cache_enabled);
        stream.put_var_u32(self.used_blob_hashes.len() as u32);
        for blob in self.used_blob_hashes.iter() {
            stream.put_u64_le(*blob);
        }
        stream.put_var_u32(self.extra_payload.len() as u32);
        stream.put(self.extra_payload.as_slice());
        /*stream.put_var_i32(self.chunk_x);
        stream.put_var_i32(self.chunk_z);
        stream.put_var_i32(self.dimension_id);
        if self.client_sub_chunk_requests_enabled {
            if self.sub_chunk_count == u32::MAX {
                stream.put_var_u32(LevelChunk::CLIENT_REQUEST_FULL_COLUMN_FAKE_COUNT);
            } else {
                stream.put_var_u32(LevelChunk::CLIENT_REQUEST_TRUNCATED_COLUMN_FAKE_COUNT);
                stream.put_u16_le(self.sub_chunk_count as u16);
            }
        } else {
            stream.put_var_u32(self.sub_chunk_count);
        }

        stream.put_bool(self.used_blob_hashes.is_some());
        if self.used_blob_hashes.is_some() {
            stream.put_var_u32(self.used_blob_hashes.clone().unwrap().len() as u32);
            for blob in self.used_blob_hashes.clone().unwrap() {
                stream.put_u64_le(blob);
            }
        }

        stream.put_var_u32(self.extra_payload.len() as u32);
        stream.put(&self.extra_payload);*/
    }

    fn decode(stream: &mut Reader) -> LevelChunk {
        let chunk_x = stream.get_var_i32();
        let chunk_z = stream.get_var_i32();
        let dimension_id = stream.get_var_i32();
        let sub_chunk_count = stream.get_var_u32();
        let client_request_sub_chunk_limit = PacketSerializer::read_optional(stream, |s| s.get_var_i32());
        let cache_enabled = stream.get_bool();

        let len = stream.get_var_u32();
        if len > Self::MAX_BLOB_HASHES {
            panic!("Expected at most {} blob hashes, got {}", Self::MAX_BLOB_HASHES, len);
        }

        let mut used_blob_hashes: Vec<u64> = Vec::with_capacity(len as usize);
        for _ in 0..len {
            used_blob_hashes.push(stream.get_u64_le());
        }

        let extra_payload = PacketSerializer::get_byte_string(stream);

        LevelChunk {
            chunk_x,
            chunk_z,
            dimension_id,
            sub_chunk_count,
            client_request_sub_chunk_limit,
            cache_enabled,
            used_blob_hashes,
            extra_payload,
        }
    }
}
