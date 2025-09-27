use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;

pub struct UpdateAdventureSettings {
    pub no_attacking_mobs: bool,
    pub no_attacking_players: bool,
    pub world_immutable: bool,
    pub show_name_tags: bool,
    pub auto_jump: bool
}

pub fn new(no_attacking_mobs: bool, no_attacking_players: bool, world_immutable: bool, show_name_tags: bool, auto_jump: bool) -> UpdateAdventureSettings {
    UpdateAdventureSettings { no_attacking_mobs, no_attacking_players, world_immutable, show_name_tags, auto_jump }
}

impl Packet for UpdateAdventureSettings {
    fn id(&self) -> u16 {
        BedrockPacketType::IDUpdateAdventureSettings.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        stream.put_bool(self.no_attacking_mobs);
        stream.put_bool(self.no_attacking_players);
        stream.put_bool(self.world_immutable);
        stream.put_bool(self.show_name_tags);
        stream.put_bool(self.auto_jump);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> UpdateAdventureSettings {
        let mut stream = Stream::new(bytes, 0);

        let no_attacking_mobs = stream.get_bool();
        let no_attacking_players = stream.get_bool();
        let world_immutable = stream.get_bool();
        let show_name_tags = stream.get_bool();
        let auto_jump = stream.get_bool();

        UpdateAdventureSettings { no_attacking_mobs, no_attacking_players, world_immutable, show_name_tags, auto_jump }
    }

    fn debug(&self) {
        println!("No Attacking Mobs: {}", self.no_attacking_mobs);
        println!("No Attacking Players: {}", self.no_attacking_players);
        println!("World Immutable: {}", self.world_immutable);
        println!("Show Name Tags: {}", self.show_name_tags);
        println!("Auto Jump: {}", self.auto_jump);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
