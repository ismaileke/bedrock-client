use crate::utils::encryption::Encryption;
use crate::protocol::bedrock::network_settings::{SNAPPY, ZLIB};
use binary_utils::binary::Stream;
use libdeflater::{CompressionLvl, Compressor};
use miniz_oxide::inflate::decompress_to_vec;
use snap::raw::{Decoder, Encoder};

pub struct GamePacket {
    pub encryption: Option<Encryption>,
    pub compress_enabled: bool,
    pub compression_type: u8
}

impl GamePacket {
    pub fn new(encryption: Option<Encryption>, compress_enabled: bool, compression_type: u8) -> GamePacket {
        GamePacket { encryption, compress_enabled, compression_type }
    }

    pub fn encode(&mut self, payload: &Vec<u8>) -> Vec<u8> {
        let mut main_stream = Stream::new(vec![0xfe], 0);

        let mut compressed = payload.clone();
        if self.compress_enabled {
            if self.compression_type == ZLIB {
                compressed = GamePacket::compress_zlib(payload);
            } else if self.compression_type == SNAPPY {
                compressed = GamePacket::compress_snappy(payload);
            }
        }

        let mut encrypted = compressed.clone();
        if let Some(ref mut encryption) = self.encryption {
            encrypted = encryption.encrypt(&compressed).expect("Game Packet Encrypt Error");
        }

        main_stream.put(encrypted);
        Vec::from(main_stream.get_buffer())
    }

    pub fn decode(&mut self, stream: &mut Stream) {
        if self.encryption.is_some() {
            *stream = Stream::new(self.decrypt(&stream.get_remaining()), 0);
        }

        if self.compress_enabled {
            let compression_type = stream.get_byte();
            //println!("Compression Type: {}", if compression_type == 0 { "ZLIB".to_string() } else if compression_type == 1 { "SNAPPY".to_string() } else { "NONE".to_string() });
            if compression_type == ZLIB { // ZLIB
                *stream = Stream::new(GamePacket::decompress_zlib(&stream.get_remaining()), 0);
            }
            if compression_type == SNAPPY { // Snappy
                *stream = Stream::new(GamePacket::decompress_snappy(&stream.get_remaining()), 0);
            }
        }
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
            return encryption.decrypt(payload).expect("Decrypt Error GamePacket");
        }
        payload.clone()
    }

    pub fn compress_zlib(payload: &Vec<u8>) -> Vec<u8> {
        let compression_level = 7;
        let min_compression_size = 256;
        let compressible = payload.len() >= min_compression_size;
        let level = if compressible { compression_level } else { 0 };

        let compression_level = CompressionLvl::new(level).expect("Invalid compression level");
        let mut compressor = Compressor::new(compression_level);

        let mut compressed_data = vec![0u8; compressor.deflate_compress_bound(payload.len())];

        let _compressed_size = compressor.deflate_compress(payload.as_slice(), &mut compressed_data).expect("Compression failed");

        let mut result = vec![0x00]; // 0x00 = ZLIB
        result.extend(compressed_data);

        result
    }

    pub fn compress_snappy(payload: &Vec<u8>) -> Vec<u8> {
        let mut encoder = Encoder::new();
        let compressed_data = encoder.compress_vec(payload.as_slice()).expect("Snappy Compression failed");

        let mut result = vec![0x01]; // 0x01 = Snappy
        result.extend(compressed_data);
        result
    }

    pub fn decompress_zlib(payload: &Vec<u8>) -> Vec<u8> {
        let decompressed_data = decompress_to_vec(payload.as_slice()).expect("ZLIB Decompress Error");
        decompressed_data
    }

    pub fn decompress_snappy(payload: &Vec<u8>) -> Vec<u8> {
        let mut decoder = Decoder::new();
        decoder.decompress_vec(payload.as_slice()).expect("Snappy Decompress Error")
    }
}
