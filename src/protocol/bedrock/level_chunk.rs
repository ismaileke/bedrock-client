use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use std::any::Any;

#[derive(serde::Serialize, Debug)]
pub struct LevelChunk {
    pub chunk_x: i32,
    pub chunk_z: i32,
    pub dimension_id: i32,
    pub highest_sub_chunk: u16,
    pub sub_chunk_count: u32,
    pub cache_enabled: bool,
    pub used_blob_hashes: Option<Vec<u64>>,
    pub extra_payload: Vec<u8>,
}

impl LevelChunk {
    pub const SUB_CHUNK_REQUEST_MODE_LIMITLESS: u32 = u32::MAX;
    
    pub const SUB_CHUNK_REQUEST_MODE_LIMITED: u32 = u32::MAX - 1;

    //this appears large enough for a world height of 1024 blocks - it may need to be increased in the future
    pub const MAX_BLOB_HASHES: u32 = 64;
}

impl Packet for LevelChunk {
    fn id(&self) -> u16 {
        BedrockPacketType::IDLevelChunk.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

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
        }*/

        stream.put_var_u32(self.extra_payload.len() as u32);
        stream.put(self.extra_payload.clone());

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> LevelChunk {
        let chunk_x = stream.get_var_i32();
        let chunk_z = stream.get_var_i32();
        let dimension_id = stream.get_var_i32();

        let sub_chunk_count = stream.get_var_u32();
        let mut highest_sub_chunk = 0;
        if sub_chunk_count == LevelChunk::SUB_CHUNK_REQUEST_MODE_LIMITED {
            highest_sub_chunk = stream.get_u16_le();
        }
        let cache_enabled = stream.get_bool();
        let mut used_blob_hashes: Option<Vec<u64>> = None;
        if cache_enabled {
            let count = stream.get_var_u32();
            if count > LevelChunk::MAX_BLOB_HASHES {
                panic!(
                    "Expected at most {} blob hashes, got {}",
                    LevelChunk::MAX_BLOB_HASHES,
                    count
                );
            } else {
                let mut blob_hashes = vec![];
                for _ in 0..count {
                    let blob = stream.get_u64_le();
                    blob_hashes.push(blob);
                }
                used_blob_hashes = Some(blob_hashes)
            }
        }

        let length = stream.get_var_u32();
        let extra_payload = stream.get(length);

        /*let sub_chunk_count: u32;
        let client_sub_chunk_requests_enabled: bool;

        let sub_chunk_count_but_not_really = stream.get_var_u32();
        if sub_chunk_count_but_not_really == LevelChunk::CLIENT_REQUEST_FULL_COLUMN_FAKE_COUNT {
            client_sub_chunk_requests_enabled = true;
            sub_chunk_count = u32::MAX;
        } else if sub_chunk_count_but_not_really == LevelChunk::CLIENT_REQUEST_TRUNCATED_COLUMN_FAKE_COUNT {
            client_sub_chunk_requests_enabled = true;
            sub_chunk_count = stream.get_u16_le() as u32;
        } else {
            client_sub_chunk_requests_enabled = false;
            sub_chunk_count = sub_chunk_count_but_not_really;
        }

        let cache_enabled = stream.get_bool();

        let mut used_blob_hashes: Option<Vec<u64>> = None;
        if cache_enabled {
            let count = stream.get_var_u32();
            if count > LevelChunk::MAX_BLOB_HASHES {
                eprintln!("Expected at most {} blob hashes, got {}", LevelChunk::MAX_BLOB_HASHES, count);
            } else {
                let mut blob_hashes = vec![];
                for _ in 0..count {
                    let blob = stream.get_u64_le();
                    blob_hashes.push(blob);
                }
                used_blob_hashes = Option::from(blob_hashes);
            }
        }

        let length = stream.get_var_u32();
        let extra_payload = stream.get(length);*/

        LevelChunk {
            chunk_x,
            chunk_z,
            dimension_id,
            highest_sub_chunk,
            sub_chunk_count,
            cache_enabled,
            used_blob_hashes,
            extra_payload,
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String { serde_json::to_string(self).unwrap() }
}
