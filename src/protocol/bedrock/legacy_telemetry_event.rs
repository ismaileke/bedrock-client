use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::Stream;
use std::any::Any;

#[derive(serde::Serialize, Debug)]
pub struct LegacyTelemetryEvent {
    pub player_unique_id: i64,
    pub event_type: i32,
    pub use_player_id: u8,
}

pub fn new(player_unique_id: i64, event_type: i32, use_player_id: u8) -> LegacyTelemetryEvent {
    LegacyTelemetryEvent {
        player_unique_id,
        event_type,
        use_player_id,
    }
}

impl Packet for LegacyTelemetryEvent {
    fn id(&self) -> u16 {
        BedrockPacketType::IDLegacyTelemetryEvent.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        PacketSerializer::put_actor_unique_id(&mut stream, self.player_unique_id);
        stream.put_var_i32(self.event_type);
        stream.put_byte(self.use_player_id);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> LegacyTelemetryEvent {
        let player_unique_id = PacketSerializer::get_actor_unique_id(stream);
        let event_type = stream.get_var_i32();
        let use_player_id = stream.get_byte();

        LegacyTelemetryEvent {
            player_unique_id,
            event_type,
            use_player_id,
        }
    }

    fn debug(&self) {
        println!("Player Unique ID: {}", self.player_unique_id);
        println!("Event Type: {}", self.event_type);
        println!("Use Player ID: {}", self.use_player_id);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl LegacyTelemetryEvent {
    pub const TYPE_ACHIEVEMENT_AWARDED: i32 = 0;
    pub const TYPE_ENTITY_INTERACT: i32 = 1;
    pub const TYPE_PORTAL_BUILT: i32 = 2;
    pub const TYPE_PORTAL_USED: i32 = 3;
    pub const TYPE_MOB_KILLED: i32 = 4;
    pub const TYPE_CAULDRON_USED: i32 = 5;
    pub const TYPE_PLAYER_DEATH: i32 = 6;
    pub const TYPE_BOSS_KILLED: i32 = 7;
    pub const TYPE_AGENT_COMMAND: i32 = 8;
    pub const TYPE_AGENT_CREATED: i32 = 9;
    pub const TYPE_PATTERN_REMOVED: i32 = 10; //???
    pub const TYPE_SLASH_COMMAND_EXECUTED: i32 = 11;
    pub const TYPE_FISH_BUCKETED: i32 = 12;
    pub const TYPE_MOB_BORN: i32 = 13;
    pub const TYPE_PET_DIED: i32 = 14;
    pub const TYPE_CAULDRON_BLOCK_USED: i32 = 15;
    pub const TYPE_COMPOSTER_BLOCK_USED: i32 = 16;
    pub const TYPE_BELL_BLOCK_USED: i32 = 17;
    pub const TYPE_ACTOR_DEFINITION: i32 = 18;
    pub const TYPE_RAID_UPDATE: i32 = 19;
    pub const TYPE_PLAYER_MOVEMENT_ANOMALY: i32 = 20; //anti cheat
    pub const TYPE_PLAYER_MOVEMENT_CORRECTED: i32 = 21;
    pub const TYPE_HONEY_HARVESTED: i32 = 22;
    pub const TYPE_TARGET_BLOCK_HIT: i32 = 23;
    pub const TYPE_PIGLIN_BARTER: i32 = 24;
    pub const TYPE_PLAYER_WAXED_OR_UNWAXED_COPPER: i32 = 25;
    pub const TYPE_CODE_BUILDER_RUNTIME_ACTION: i32 = 26;
    pub const TYPE_CODE_BUILDER_SCOREBOARD: i32 = 27;
    pub const TYPE_STRIDER_RIDDEN_IN_LAVA_IN_OVERWORLD: i32 = 28;
    pub const TYPE_SNEAK_CLOSE_TO_SCULK_SENSOR: i32 = 29;
    pub const TYPE_CAREFUL_RESTORATION: i32 = 30;
    pub const TYPE_ITEM_USED: i32 = 31;
}
