use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::Stream;
use std::any::Any;

#[derive(serde::Serialize, Debug)]
pub struct UpdatePlayerGameType {
    pub game_mode: i32,
    pub player_actor_unique_id: i64,
    pub tick: u64,
}

impl Packet for UpdatePlayerGameType {
    fn id(&self) -> u16 {
        BedrockPacketType::IDUpdatePlayerGameType.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_var_i32(self.game_mode);
        PacketSerializer::put_actor_unique_id(&mut stream, self.player_actor_unique_id);
        stream.put_var_u64(self.tick);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> UpdatePlayerGameType {
        let game_mode = stream.get_var_i32();
        let player_actor_unique_id = PacketSerializer::get_actor_unique_id(stream);
        let tick = stream.get_var_u64();

        UpdatePlayerGameType {
            game_mode,
            player_actor_unique_id,
            tick,
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String { serde_json::to_string(self).unwrap() }
}

impl UpdatePlayerGameType {
    pub const SURVIVAL: i32 = 0;
    pub const CREATIVE: i32 = 1;
    pub const ADVENTURE: i32 = 2;
    pub const SURVIVAL_VIEWER: i32 = 3;
    pub const CREATIVE_VIEWER: i32 = 4;
    pub const DEFAULT: i32 = 5;
}
