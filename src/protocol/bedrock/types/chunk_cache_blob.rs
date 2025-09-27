#[derive(Debug)]
pub struct ChunkCacheBlob {
    hash: i64,
    payload: String
}

impl ChunkCacheBlob {
    pub fn new(hash: i64, payload: String) -> Self {
        ChunkCacheBlob{ hash, payload }
    }

    pub fn get_hash(&self) -> i64 {
        self.hash
    }

    pub fn get_payload(&self) -> String {
        self.payload.clone()
    }
}
