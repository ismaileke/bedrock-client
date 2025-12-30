use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::Stream;
use std::any::Any;

#[derive(serde::Serialize, Debug)]
pub struct ServerBoundLoadingScreen {
    pub loading_screen_type: i32, //see types/hud/loading_screen_type.rs
    pub loading_screen_id: Option<u32>,
}

impl Packet for ServerBoundLoadingScreen {
    fn id(&self) -> u16 {
        BedrockPacketType::IDServerBoundLoadingScreen.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_var_i32(self.loading_screen_type);
        PacketSerializer::write_optional(&mut stream, &self.loading_screen_id, |s, v| {
            s.put_u32_le(*v)
        });

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> ServerBoundLoadingScreen {
        let loading_screen_type = stream.get_var_i32();
        let loading_screen_id = PacketSerializer::read_optional(stream, |s| s.get_u32_le());

        ServerBoundLoadingScreen { loading_screen_type, loading_screen_id }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String { serde_json::to_string(self).unwrap() }
}
