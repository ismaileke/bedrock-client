use binary_utils::binary::Stream;

pub struct LevelChunk {
    pub chunk_x: i32,
    pub chunk_z: i32,
    pub dimension_id: i32,
    pub sub_chunk_count: u32,
    pub client_sub_chunk_requests_enabled: bool,
    pub used_blob_hashes: Option<Vec<i64>>,
    pub extra_payload: Vec<u8>
}

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

pub fn decode(bytes: Vec<u8>) -> LevelChunk {
    let mut stream = Stream::new(bytes, 0);

    let chunk_x = stream.get_var_int();
    let chunk_z = stream.get_var_int();
    let dimension_id = stream.get_var_int();

    let sub_chunk_count: u32;
    let client_sub_chunk_requests_enabled: bool;


    let sub_chunk_count_but_not_really = stream.get_unsigned_var_int();
    if sub_chunk_count_but_not_really == CLIENT_REQUEST_FULL_COLUMN_FAKE_COUNT {
        client_sub_chunk_requests_enabled = true;
        sub_chunk_count = u32::MAX;
    } else if sub_chunk_count_but_not_really == CLIENT_REQUEST_TRUNCATED_COLUMN_FAKE_COUNT {
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
        if count > MAX_BLOB_HASHES {
            eprintln!("Expected at most {} blob hashes, got {}", MAX_BLOB_HASHES, count);
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