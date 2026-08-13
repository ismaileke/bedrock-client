use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::{Reader, Writer};

pub const COMPRESS_NOTHING: u8 = 0;
pub const COMPRESS_EVERYTHING: u8 = 1;

pub const ZLIB: u8 = 0;
pub const SNAPPY: u8 = 1;
pub const NONE: u8 = 255;

#[derive(serde::Serialize, Debug)]
pub struct NetworkSettings {
    pub compression_threshold: u16,
    pub compression_algorithm: u16,
    pub enable_client_throttling: bool,
    pub client_throttle_threshold: u8,
    pub client_throttle_scalar: f32,
}

impl Packet for NetworkSettings {
    fn id(&self) -> u16 {
        BedrockPacketType::IDNetworkSettings.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_u16_le(self.compression_threshold);
        stream.put_u16_le(self.compression_algorithm);
        stream.put_bool(self.enable_client_throttling);
        stream.put_u8(self.client_throttle_threshold);
        stream.put_f32_le(self.client_throttle_scalar);
    }

    fn decode(stream: &mut Reader) -> NetworkSettings {
        let compression_threshold = stream.get_u16_le();
        let compression_algorithm = stream.get_u16_le();
        let enable_client_throttling = stream.get_bool();
        let client_throttle_threshold = stream.get_u8();
        let client_throttle_scalar = stream.get_f32_le();

        NetworkSettings {
            compression_threshold,
            compression_algorithm,
            enable_client_throttling,
            client_throttle_threshold,
            client_throttle_scalar,
        }
    }
}
