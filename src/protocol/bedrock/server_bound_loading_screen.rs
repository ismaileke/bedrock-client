use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct ServerBoundLoadingScreen {
    pub loading_screen_type: i32, //see types/hud/loading_screen_type.rs
    pub loading_screen_id: Option<u32>,
}

impl Packet for ServerBoundLoadingScreen {
    fn id(&self) -> u16 {
        BedrockPacketType::IDServerBoundLoadingScreen.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_var_i32(self.loading_screen_type);
        PacketSerializer::write_optional(stream, &self.loading_screen_id, |s, v| {
            s.put_u32_le(*v)
        });
    }

    fn decode(stream: &mut Reader) -> ServerBoundLoadingScreen {
        let loading_screen_type = stream.get_var_i32();
        let loading_screen_id = PacketSerializer::read_optional(stream, |s| s.get_u32_le());

        ServerBoundLoadingScreen { loading_screen_type, loading_screen_id }
    }
}
