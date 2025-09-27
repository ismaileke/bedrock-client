use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

pub struct ServerBoundLoadingScreen {
    pub loading_screen_type: i32, //see types/hud/loading_screen_type.rs
    pub loading_screen_id: Option<u32>
}

pub fn new(loading_screen_type: i32, loading_screen_id: Option<u32>) -> ServerBoundLoadingScreen {
    ServerBoundLoadingScreen { loading_screen_type, loading_screen_id }
}

impl Packet for ServerBoundLoadingScreen {
    fn id(&self) -> u16 {
        BedrockPacketType::IDServerBoundLoadingScreen.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        stream.put_var_int(self.loading_screen_type);
        PacketSerializer::write_optional(&mut stream, &self.loading_screen_id, |s, v| s.put_l_int(*v));

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> ServerBoundLoadingScreen {
        let mut stream = Stream::new(bytes, 0);

        let loading_screen_type = stream.get_var_int();
        let loading_screen_id = PacketSerializer::read_optional(&mut stream, |s| s.get_l_int());

        ServerBoundLoadingScreen { loading_screen_type, loading_screen_id }
    }

    fn debug(&self) {
        println!("Loading Screen Type: {}", self.loading_screen_type);
        println!("Loading Screen ID: {:?}", self.loading_screen_id);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
