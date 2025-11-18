use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;

pub struct ClientCacheBlobStatus {
    pub miss_hashes: Vec<u64>,
    pub hit_hashes: Vec<u64>
}

pub fn new(miss_hashes: Vec<u64>, hit_hashes: Vec<u64>) -> ClientCacheBlobStatus {
    ClientCacheBlobStatus { miss_hashes, hit_hashes }
}

impl Packet for ClientCacheBlobStatus {
    fn id(&self) -> u16 {
        BedrockPacketType::IDClientCacheBlobStatus.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_var_u32(self.miss_hashes.len() as u32);
        stream.put_var_u32(self.hit_hashes.len() as u32);
        for hash in self.miss_hashes.iter() {
            stream.put_u64_le(*hash);
        }
        for hash in self.hit_hashes.iter() {
            stream.put_u64_le(*hash);
        }

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(bytes: Vec<u8>) -> ClientCacheBlobStatus {
        let mut stream = Stream::new(bytes, 0);

        let miss_len = stream.get_var_u32() as usize;
        let hit_len = stream.get_var_u32() as usize;
        let mut miss_hashes = Vec::new();
        let mut hit_hashes = Vec::new();
        for _ in 0..miss_len {
            miss_hashes.push(stream.get_u64_le());
        }
        for _ in 0..hit_len {
            hit_hashes.push(stream.get_u64_le());
        }

        ClientCacheBlobStatus { miss_hashes, hit_hashes }
    }

    fn debug(&self) {
        println!("Miss Hashes: {:?}", self.miss_hashes);
        println!("Hit Hashes: {:?}", self.hit_hashes);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
