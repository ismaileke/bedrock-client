use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;

pub struct ClientCacheBlobStatus {
    pub miss_hashes: Vec<i64>,
    pub hit_hashes: Vec<i64>
}

pub fn new(miss_hashes: Vec<i64>, hit_hashes: Vec<i64>) -> ClientCacheBlobStatus {
    ClientCacheBlobStatus { miss_hashes, hit_hashes }
}

impl Packet for ClientCacheBlobStatus {
    fn id(&self) -> u16 {
        BedrockPacketType::IDClientCacheBlobStatus.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        stream.put_unsigned_var_int(self.miss_hashes.len() as u32);
        stream.put_unsigned_var_int(self.hit_hashes.len() as u32);
        for hash in self.miss_hashes.iter() {
            stream.put_l_long(*hash);
        }
        for hash in self.hit_hashes.iter() {
            stream.put_l_long(*hash);
        }

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> ClientCacheBlobStatus {
        let mut stream = Stream::new(bytes, 0);

        let miss_len = stream.get_unsigned_var_int() as usize;
        let hit_len = stream.get_unsigned_var_int() as usize;
        let mut miss_hashes = Vec::new();
        let mut hit_hashes = Vec::new();
        for _ in 0..miss_len {
            miss_hashes.push(stream.get_l_long());
        }
        for _ in 0..hit_len {
            hit_hashes.push(stream.get_l_long());
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
