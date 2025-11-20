use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

pub struct ShowCredits {
    pub player_actor_runtime_id: u64,
    pub status: i32
}

pub fn new(player_actor_runtime_id: u64, status: i32) -> ShowCredits {
    ShowCredits { player_actor_runtime_id, status }
}

impl Packet for ShowCredits {
    fn id(&self) -> u16 {
        BedrockPacketType::IDShowCredits.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        PacketSerializer::put_actor_runtime_id(&mut stream, self.player_actor_runtime_id);
        stream.put_var_i32(self.status);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> ShowCredits {
        let player_actor_runtime_id = PacketSerializer::get_actor_runtime_id(stream);
        let status = stream.get_var_i32();

        ShowCredits { player_actor_runtime_id, status }
    }

    fn debug(&self) {
        println!("Player Actor Runtime ID: {}", self.player_actor_runtime_id);
        println!("Status: {}", self.status);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ShowCredits {
    pub const STATUS_START_CREDITS: i32 = 0;
    pub const STATUS_END_CREDITS: i32 = 1;
}
