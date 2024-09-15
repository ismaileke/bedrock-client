use crate::utils::encryption::Encryption;
use binary_utils::binary::Stream;
use libdeflater::{CompressionLvl, Compressor};
use miniz_oxide::inflate::decompress_to_vec;

pub struct GamePacket {
    pub encryption: Encryption,
}

pub fn new(encryption: Encryption) -> GamePacket {
    GamePacket{ encryption }
}

impl GamePacket {

    pub fn encrypt(&mut self, payload: &Vec<u8>) -> Vec<u8> {
        let mut main_stream = Stream::new(vec![0xfe], 0);
        let compressed = compress(payload);
        let encrypted = self.encryption.encrypt(&compressed).expect("GamePacket Encrypt Error");
        main_stream.put(encrypted);
        main_stream.get_buffer()
    }

    pub fn decrypt(&mut self, payload: &Vec<u8>) -> Vec<u8> {
        self.encryption.decrypt(payload).expect("Decrypt Error GamePacket")
    }
}

pub fn compress(payload: &Vec<u8>) -> Vec<u8> {
    let compression_level = 7;
    let min_compression_size = 256;
    let compressible = payload.len() >= min_compression_size;
    let level = if compressible { compression_level } else { 0 };

    let compression_level = CompressionLvl::new(level).expect("Invalid compression level");
    let mut compressor = Compressor::new(compression_level);

    let mut compressed_data = vec![0u8; compressor.deflate_compress_bound(payload.len())];

    let _compressed_size = compressor
        .deflate_compress(payload.as_slice(), &mut compressed_data)
        .expect("Compression failed");

    let mut result = vec![0x00];
    result.extend(compressed_data);

    result
}

pub fn decompress(payload: &Vec<u8>) -> Vec<u8> {
    let decompressed_data = decompress_to_vec(payload.as_slice()).expect("DecompressToVec Error");
    decompressed_data
}
