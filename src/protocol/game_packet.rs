use crate::utils::encryption::Encryption;
use binary_utils::binary::Stream;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use miniz_oxide::inflate::decompress_to_vec;
use std::io::Write;

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
    let level = if compressible { Compression::new(compression_level) } else { Compression::new(0) };

    let mut encoder = ZlibEncoder::new(Vec::new(), level);

    encoder.write_all(payload.as_slice()).expect("ZLIB ENCODE ERROR");
    let compressed_data = encoder.finish().expect("ZLIB ENCODE ERROR 2");

    let mut result = vec![0]; // Compression Algorithm (ZLIB=0)
    result.extend(compressed_data);
    result
}

pub fn decompress(payload: &Vec<u8>) -> Vec<u8> {
    /*let mut decoder = GzDecoder::new(payload.as_slice());
    let mut decompressed = Vec::new();

    match decoder.read_to_end(&mut decompressed) {
        Ok(_) => decompressed.to_vec(),
        Err(_) => payload.to_vec()
    }*/
    let decompressed_data = decompress_to_vec(payload.as_slice()).expect("DecompressToVec Error");
    decompressed_data
}
