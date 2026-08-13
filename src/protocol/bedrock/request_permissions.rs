use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct RequestPermissions {
    pub target_actor_unique_id: i64,
    pub player_permission: i32, //see types/player_permissions.rs
    pub custom_flags: u16,
}

impl Packet for RequestPermissions {
    fn id(&self) -> u16 {
        BedrockPacketType::IDRequestPermissions.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_i64_le(self.target_actor_unique_id);
        stream.put_var_i32(self.player_permission);
        stream.put_u16_le(self.custom_flags);
    }

    fn decode(stream: &mut Reader) -> RequestPermissions {
        let target_actor_unique_id = stream.get_i64_le();
        let player_permission = stream.get_var_i32();
        let custom_flags = stream.get_u16_le();

        RequestPermissions {
            target_actor_unique_id,
            player_permission,
            custom_flags,
        }
    }
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
