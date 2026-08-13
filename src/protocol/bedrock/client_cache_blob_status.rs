use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct ClientCacheBlobStatus {
    pub miss_hashes: Vec<u64>,
    pub hit_hashes: Vec<u64>,
}

impl Packet for ClientCacheBlobStatus {
    fn id(&self) -> u16 {
        BedrockPacketType::IDClientCacheBlobStatus.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_var_u32(self.miss_hashes.len() as u32);
        for hash in self.miss_hashes.iter() {
            stream.put_u64_le(*hash);
        }
        stream.put_var_u32(self.hit_hashes.len() as u32);
        for hash in self.hit_hashes.iter() {
            stream.put_u64_le(*hash);
        }
    }

    fn decode(stream: &mut Reader) -> ClientCacheBlobStatus {
        let miss_len = stream.get_var_u32() as usize;
        let mut miss_hashes = Vec::new();
        for _ in 0..miss_len {
            miss_hashes.push(stream.get_u64_le());
        }
        let hit_len = stream.get_var_u32() as usize;
        let mut hit_hashes = Vec::new();
        for _ in 0..hit_len {
            hit_hashes.push(stream.get_u64_le());
        }

        ClientCacheBlobStatus { miss_hashes, hit_hashes }
    }
}
