use binary_utils::binary::Stream;
use log::error;
const MAX_SAVED_CHUNKS: u32 = 9216;

pub struct NetworkChunkPublisherUpdate {
    pub block_pos: Vec<i32>,
    pub radius: u32,
    pub saved_chunks: Vec<Vec<i32>>
}

pub fn decode(bytes: Vec<u8>) -> NetworkChunkPublisherUpdate {
    let mut stream = Stream::new(bytes, 0);

    let x = stream.get_var_int();
    let y = stream.get_var_int();
    let z = stream.get_var_int();
    let block_pos = vec![x, y, z];

    let radius = stream.get_unsigned_var_int();

    let count = stream.get_l_int();

    if count > MAX_SAVED_CHUNKS {
        error!("Expected at most {} saved chunks, got {}", MAX_SAVED_CHUNKS, count)
    }

    let mut saved_chunks = vec![];
    for _ in 0..count {
        let chunk_x = stream.get_var_int();
        let chunk_z = stream.get_var_int();
        saved_chunks.push(vec![chunk_x, chunk_z]);
    }

    NetworkChunkPublisherUpdate { block_pos, radius, saved_chunks }
}