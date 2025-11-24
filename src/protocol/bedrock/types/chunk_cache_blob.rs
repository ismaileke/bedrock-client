#[derive(serde::Serialize, Debug)]
pub struct ChunkCacheBlob {
    hash: u64,
    payload: String
}

impl ChunkCacheBlob {
    pub fn new(hash: u64, payload: String) -> Self {
        ChunkCacheBlob{ hash, payload }
    }

    pub fn get_hash(&self) -> u64 {
        self.hash
    }

    pub fn get_payload(&self) -> String {
        self.payload.clone()
    }
}
