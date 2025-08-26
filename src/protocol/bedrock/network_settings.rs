use binary_utils::binary::Stream;

pub const COMPRESS_NOTHING: u8 = 0;
pub const COMPRESS_EVERYTHING: u8 = 1;

pub const ZLIB: u8 = 0;
pub const SNAPPY: u8 = 1;
pub const NONE: u8 = 255;

pub struct NetworkSettings {
    pub compression_threshold: u16,
    pub compression_algorithm: u16,
    pub enable_client_throttling: bool,
    pub client_throttle_threshold: u8,
    pub client_throttle_scalar: f32
}

impl NetworkSettings {
    pub fn debug(&self) {
        println!("Compression Threshold: {}", if self.compression_threshold == 1 { "COMPRESS_EVERYTHING" } else { "COMPRESS_NOTHING" });
        println!("Compression Algorithm: {}", if self.compression_algorithm == 0 { "ZLIB" } else if self.compression_algorithm == 1 { "SNAPPY" } else { "NONE" });
        println!("Enable Client Throttling: {}", self.enable_client_throttling);
        println!("Client Throttle Threshold: {}", self.client_throttle_threshold);
        println!("Client Throttle Scalar: {}", self.client_throttle_scalar);
    }
}

pub fn decode(bytes: Vec<u8>) -> NetworkSettings {
    let mut stream = Stream::new(bytes, 0);

    let compression_threshold = stream.get_l_short();
    let compression_algorithm = stream.get_l_short();
    let enable_client_throttling = stream.get_bool();
    let client_throttle_threshold = stream.get_byte();
    let client_throttle_scalar = stream.get_l_float();

    NetworkSettings { compression_threshold, compression_algorithm, enable_client_throttling, client_throttle_threshold, client_throttle_scalar }
}