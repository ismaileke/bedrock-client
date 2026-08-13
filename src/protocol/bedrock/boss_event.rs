use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct BossEvent {
    pub boss_actor_unique_id: i64,
    pub player_actor_unique_id: i64,
    pub event_type: u8,
    pub title: String,
    pub filtered_title: String,
    pub health_percent: f32,
    pub color: u8,
    pub overlay: u8,
}

impl BossEvent {
    /** S2C: Shows the boss-bar to the player. */
    pub const TYPE_SHOW: u8 = 0;
    /** C2S: Registers a player to a boss fight. */
    pub const TYPE_REGISTER_PLAYER: u8 = 1;
    /** S2C: Removes the boss-bar from the client. */
    pub const TYPE_HIDE: u8 = 2;
    /** C2S: Unregisters a player from a boss fight. */
    pub const TYPE_UNREGISTER_PLAYER: u8 = 3;
    /** S2C: Sets the bar percentage. */
    pub const TYPE_HEALTH_PERCENT: u8 = 4;
    /** S2C: Sets the title of the bar. */
    pub const TYPE_TITLE: u8 = 5;
    /** S2C: Updates misc properties of the bar and environment. */
    pub const TYPE_PROPERTIES: u8 = 6;
    /** S2C: Updates boss-bar color and overlay texture. */
    pub const TYPE_TEXTURE: u8 = 7;
    /** C2S: Client asking the server to resend all boss data. */
    pub const TYPE_QUERY: u8 = 8;

    pub const PINK: u8 = 0;
    pub const BLUE: u8 = 1;
    pub const RED: u8 = 2;
    pub const GREEN: u8 = 3;
    pub const YELLOW: u8 = 4;
    pub const PURPLE: u8 = 5;
    pub const REBECCA_PURPLE: u8 = 6;
    pub const WHITE: u8 = 7;
}

impl Packet for BossEvent {
    fn id(&self) -> u16 {
        BedrockPacketType::IDBossEvent.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_actor_unique_id(stream, self.boss_actor_unique_id);
        PacketSerializer::put_actor_unique_id(stream, self.player_actor_unique_id);
        stream.put_u8(self.event_type);
        PacketSerializer::put_string(stream, self.title.clone());
        PacketSerializer::put_string(stream, self.filtered_title.clone());
        stream.put_f32_le(self.health_percent);
        stream.put_u8(self.color);
        stream.put_u8(self.overlay);
    }

    fn decode(stream: &mut Reader) -> BossEvent {
        let boss_actor_unique_id = PacketSerializer::get_actor_unique_id(stream);
        let player_actor_unique_id = PacketSerializer::get_actor_unique_id(stream);
        let event_type = stream.get_u8();
        let title = PacketSerializer::get_string(stream);
        let filtered_title = PacketSerializer::get_string(stream);
        let health_percent = stream.get_f32_le();
        let color = stream.get_u8();
        let overlay = stream.get_u8();

        BossEvent {
            boss_actor_unique_id,
            player_actor_unique_id,
            event_type,
            title,
            filtered_title,
            health_percent,
            color,
            overlay,
        }
    }
}
