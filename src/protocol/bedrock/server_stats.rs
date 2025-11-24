use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct ServerStats {
    pub server_time: f32,
    pub network_time: f32
}

pub fn new(server_time: f32, network_time: f32) -> ServerStats {
    ServerStats { server_time, network_time }
}

impl Packet for ServerStats {
    fn id(&self) -> u16 {
        BedrockPacketType::IDServerStats.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_f32_le(self.server_time);
        stream.put_f32_le(self.network_time);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> ServerStats {
        let server_time = stream.get_f32_le();
        let network_time = stream.get_f32_le();

        ServerStats { server_time, network_time }
    }

    fn debug(&self) {
        println!("Server Time: {}", self.server_time);
        println!("Network Time: {}", self.network_time);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
