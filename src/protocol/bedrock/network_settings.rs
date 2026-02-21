use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::raknet::packet_ids::PacketType;
use binary_utils::binary::Stream;
use std::any::Any;

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
        BedrockPacketType::IDNetworkSettings.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_u16_le(self.compression_threshold);
        stream.put_u16_le(self.compression_algorithm);
        stream.put_bool(self.enable_client_throttling);
        stream.put_byte(self.client_throttle_threshold);
        stream.put_f32_le(self.client_throttle_scalar);

        let mut main_stream = Stream::new(Vec::new(), 0);
        main_stream.put_byte(PacketType::Game.get_byte());
        main_stream.put_var_u32(stream.get_buffer().len() as u32);
        main_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(main_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> NetworkSettings {
        let compression_threshold = stream.get_u16_le();
        let compression_algorithm = stream.get_u16_le();
        let enable_client_throttling = stream.get_bool();
        let client_throttle_threshold = stream.get_byte();
        let client_throttle_scalar = stream.get_f32_le();

        NetworkSettings {
            compression_threshold,
            compression_algorithm,
            enable_client_throttling,
            client_throttle_threshold,
            client_throttle_scalar,
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String { serde_json::to_string(self).unwrap() }
}
