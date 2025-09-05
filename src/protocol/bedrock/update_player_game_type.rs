use binary_utils::binary::Stream;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;

pub const SURVIVAL: i32 = 0;
pub const CREATIVE: i32 = 1;
pub const ADVENTURE: i32 = 2;
pub const SURVIVAL_VIEWER: i32 = 3;
pub const CREATIVE_VIEWER: i32 = 4;
pub const DEFAULT: i32 = 5;

pub struct UpdatePlayerGameType {
    pub game_mode: i32,
    pub player_actor_unique_id: i64,
    pub tick: u64
}

pub fn new(game_mode: i32, player_actor_unique_id: i64, tick: u64) -> UpdatePlayerGameType {
    UpdatePlayerGameType{ game_mode, player_actor_unique_id, tick }
}

impl UpdatePlayerGameType {
    pub fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(BedrockPacketType::get_byte(BedrockPacketType::UpdatePlayerGameType) as u32);

        stream.put_var_int(self.game_mode);
        stream.put_var_long(self.player_actor_unique_id);
        stream.put_unsigned_var_long(self.tick);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    pub fn debug(&self) {
        println!("Game mode: {}", self.game_mode);
        println!("Player actor unique id: {}", self.player_actor_unique_id);
        println!("Tick: {}", self.tick);
    }
}

pub fn decode(bytes: Vec<u8>) -> UpdatePlayerGameType {
    let mut stream = Stream::new(bytes, 0);

    let game_mode = stream.get_var_int();
    let player_actor_unique_id = stream.get_var_long();
    let tick = stream.get_unsigned_var_long();


    UpdatePlayerGameType { game_mode, player_actor_unique_id, tick }
}