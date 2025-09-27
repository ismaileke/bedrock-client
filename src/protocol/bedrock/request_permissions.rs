use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;

pub struct RequestPermissions {
    pub target_actor_unique_id: i64,
    pub player_permission: i32, //see types/player_permissions.rs
    pub custom_flags: u16
}

pub fn new(target_actor_unique_id: i64, player_permission: i32, custom_flags: u16) -> RequestPermissions {
    RequestPermissions { target_actor_unique_id, player_permission, custom_flags }
}

impl RequestPermissions {
    pub const FLAG_BUILD: u16 = 1 << 0;
    pub const FLAG_MINE: u16 = 1 << 1;
    pub const FLAG_DOORS_AND_SWITCHES: u16 = 1 << 2;
    pub const FLAG_OPEN_CONTAINERS: u16 = 1 << 3;
    pub const FLAG_ATTACK_PLAYERS: u16 = 1 << 4;
    pub const FLAG_ATTACK_MOBS: u16 = 1 << 5;
    pub const FLAG_OPERATOR: u16 = 1 << 6;
    pub const FLAG_TELEPORT: u16 = 1 << 7;
}

impl Packet for RequestPermissions {
    fn id(&self) -> u16 {
        BedrockPacketType::IDRequestPermissions.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        stream.put_l_long(self.target_actor_unique_id);
        stream.put_var_int(self.player_permission);
        stream.put_l_short(self.custom_flags);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> RequestPermissions {
        let mut stream = Stream::new(bytes, 0);

        let target_actor_unique_id = stream.get_l_long();
        let player_permission = stream.get_var_int();
        let custom_flags = stream.get_l_short();

        RequestPermissions { target_actor_unique_id, player_permission, custom_flags }
    }

    fn debug(&self) {
        println!("Target Actor Unique ID: {}", self.target_actor_unique_id);
        println!("Player Permission: {}", self.player_permission);
        println!("Custom Flags: {}", self.custom_flags);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
