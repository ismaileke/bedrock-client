use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;

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
        stream.put_unsigned_var_int(self.id() as u32);

        stream.put_l_float(self.server_time);
        stream.put_l_float(self.network_time);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> ServerStats {
        let mut stream = Stream::new(bytes, 0);

        let server_time = stream.get_l_float();
        let network_time = stream.get_l_float();

        ServerStats { server_time, network_time }
    }

    fn debug(&self) {
        println!("Server Time: {}", self.server_time);
        println!("Network Time: {}", self.network_time);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
