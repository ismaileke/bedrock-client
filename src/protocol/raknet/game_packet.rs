use crate::utils::encryption::Encryption;
use binary_utils::binary::Stream;
use libdeflater::{CompressionLvl, Compressor};
use miniz_oxide::inflate::decompress_to_vec;

pub struct GamePacket {
    pub encryption: Option<Encryption>,
    pub compress_enabled: bool,
}

impl GamePacket {
    pub fn new(encryption: Option<Encryption>, compress_enabled: bool) -> GamePacket {
        GamePacket {
            encryption,
            compress_enabled,
        }
    }

    pub fn encode(&mut self, payload: &Vec<u8>) -> Vec<u8> {
        let mut main_stream = Stream::new(vec![0xfe], 0);

        let mut compressed = payload.clone();
        if self.compress_enabled {
            compressed = GamePacket::compress(payload);
        }

        let mut encrypted = compressed.clone();
        if let Some(ref mut encryption) = self.encryption {
            encrypted = encryption
                .encrypt(&compressed)
                .expect("Game Packet Encrypt Error");
        }

        main_stream.put(encrypted);
        Vec::from(main_stream.get_buffer())
    }

    /*pub fn encrypt(&mut self, payload: &Vec<u8>) -> Vec<u8> {
        let mut main_stream = Stream::new(vec![0xfe], 0);
        let compressed = GamePacket::compress(payload);
        let encrypted = self.encryption.encrypt(&compressed).expect("GamePacket Encrypt Error");
        main_stream.put(encrypted);
        main_stream.get_buffer()
    }*/

    pub fn decrypt(&mut self, payload: &Vec<u8>) -> Vec<u8> {
        if let Some(ref mut encryption) = self.encryption {
            return encryption
                .decrypt(payload)
                .expect("Decrypt Error GamePacket");
        }
        payload.clone()
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

        let mut result = vec![0x00]; // 0 = ZLIB
        result.extend(compressed_data);

        result
    }

    pub fn decompress(payload: &Vec<u8>) -> Vec<u8> {
        let decompressed_data =
            decompress_to_vec(payload.as_slice()).expect("DecompressToVec Error");
        decompressed_data
    }
}
