use std::any::Any;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;

pub struct LevelChunk {
    pub chunk_x: i32,
    pub chunk_z: i32,
    pub dimension_id: i32,
    pub sub_chunk_count: u32,
    pub client_sub_chunk_requests_enabled: bool,
    pub used_blob_hashes: Option<Vec<i64>>,
    pub extra_payload: Vec<u8>
}

impl LevelChunk {
    /**
    * Client will request all sub chunks as needed up to the top of the world
    */
    pub const CLIENT_REQUEST_FULL_COLUMN_FAKE_COUNT: u32 = u32::MAX;
    /**
     * Client will request sub chunks as needed up to the height written in the packet, and assume that anything above
     * that height is air (wtf mojang ...)
     */
    pub const CLIENT_REQUEST_TRUNCATED_COLUMN_FAKE_COUNT: u32 = u32::MAX - 1;

    //this appears large enough for a world height of 1024 blocks - it may need to be increased in the future
    pub const MAX_BLOB_HASHES: u32 = 64;
}

impl Packet for LevelChunk {
    fn id(&self) -> u16 {
        BedrockPacketType::IDLevelChunk.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        stream.put_var_int(self.chunk_x);
        stream.put_var_int(self.chunk_z);
        stream.put_var_int(self.dimension_id);
        if self.client_sub_chunk_requests_enabled {
            if self.sub_chunk_count == u32::MAX {
                stream.put_unsigned_var_int(LevelChunk::CLIENT_REQUEST_FULL_COLUMN_FAKE_COUNT);
            } else {
                stream.put_unsigned_var_int(LevelChunk::CLIENT_REQUEST_TRUNCATED_COLUMN_FAKE_COUNT);
            }
        } else {
            stream.put_unsigned_var_int(self.sub_chunk_count);
        }

        stream.put_bool(self.used_blob_hashes.is_some());
        if self.used_blob_hashes.is_some() {
            stream.put_unsigned_var_int(self.used_blob_hashes.clone().unwrap().len() as u32);
            for blob in self.used_blob_hashes.clone().unwrap() {
                stream.put_l_long(blob);
            }
        }

        stream.put_unsigned_var_int(self.extra_payload.len() as u32);
        stream.put(self.extra_payload.clone());

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> LevelChunk {
        let mut stream = Stream::new(bytes, 0);

        let chunk_x = stream.get_var_int();
        let chunk_z = stream.get_var_int();
        let dimension_id = stream.get_var_int();

        let sub_chunk_count: u32;
        let client_sub_chunk_requests_enabled: bool;

        let sub_chunk_count_but_not_really = stream.get_unsigned_var_int();
        if sub_chunk_count_but_not_really == LevelChunk::CLIENT_REQUEST_FULL_COLUMN_FAKE_COUNT {
            client_sub_chunk_requests_enabled = true;
            sub_chunk_count = u32::MAX;
        } else if sub_chunk_count_but_not_really == LevelChunk::CLIENT_REQUEST_TRUNCATED_COLUMN_FAKE_COUNT {
            client_sub_chunk_requests_enabled = true;
            sub_chunk_count = stream.get_l_short() as u32;
        } else {
            client_sub_chunk_requests_enabled = false;
            sub_chunk_count = sub_chunk_count_but_not_really;
        }

        let cache_enabled = stream.get_bool();

        let mut used_blob_hashes: Option<Vec<i64>> = None;
        if cache_enabled {
            let count = stream.get_unsigned_var_int();
            if count > LevelChunk::MAX_BLOB_HASHES {
                eprintln!("Expected at most {} blob hashes, got {}", LevelChunk::MAX_BLOB_HASHES, count);
            } else {
                let mut blob_hashes = vec![];
                for _ in 0..count {
                    let blob = stream.get_l_long();
                    blob_hashes.push(blob);
                }
                used_blob_hashes = Option::from(blob_hashes);
            }
        }

        let length = stream.get_unsigned_var_int();
        let extra_payload = stream.get(length).unwrap();

        LevelChunk { chunk_x, chunk_z, dimension_id, sub_chunk_count, client_sub_chunk_requests_enabled, used_blob_hashes, extra_payload }
    }

    fn debug(&self) {
        println!("Chunk X: {}", self.chunk_x);
        println!("Chunk Z: {}", self.chunk_z);
        println!("Dimension ID: {}", self.dimension_id);
        println!("Sub Chunk Count: {}", self.sub_chunk_count);
        println!("Client Sub Chunk Requests Enabled: {}", self.client_sub_chunk_requests_enabled);
        println!("Used Blob Hashes: {:?}", self.used_blob_hashes);
        println!("Extra Payload (Length): {}", self.extra_payload.len());
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
