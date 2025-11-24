use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

#[derive(serde::Serialize, Debug)]
pub struct BossEvent {
    pub boss_actor_unique_id: i64,
    pub event_type: u32,
    pub player_actor_unique_id: i64,
    pub health_percent: f32,
    pub title: String,
    pub filtered_title: String,
    pub darken_screen: bool,
    pub color: u32,
    pub overlay: u32
}

pub fn new(
    boss_actor_unique_id: i64,
    event_type: u32,
    player_actor_unique_id: i64,
    health_percent: f32,
    title: String,
    filtered_title: String,
    darken_screen: bool,
    color: u32,
    overlay: u32
) -> BossEvent {
    BossEvent { boss_actor_unique_id, event_type, player_actor_unique_id, health_percent, title, filtered_title, darken_screen, color, overlay }
}

impl Packet for BossEvent {
    fn id(&self) -> u16 {
        BedrockPacketType::IDBossEvent.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        PacketSerializer::put_actor_unique_id(&mut stream, self.boss_actor_unique_id);
        stream.put_var_u32(self.event_type);
        match self.event_type {
            BossEvent::TYPE_REGISTER_PLAYER | BossEvent::TYPE_UNREGISTER_PLAYER | BossEvent::TYPE_QUERY => {
                PacketSerializer::put_actor_unique_id(&mut stream, self.player_actor_unique_id);
            }
            BossEvent::TYPE_SHOW => {
                PacketSerializer::put_string(&mut stream, self.title.clone());
                PacketSerializer::put_string(&mut stream, self.filtered_title.clone());
                stream.put_f32_le(self.health_percent);

                stream.put_u16_le(if self.darken_screen { 1 } else { 0 });
                stream.put_var_u32(self.color);
                stream.put_var_u32(self.overlay);
            }
            BossEvent::TYPE_PROPERTIES => {
                stream.put_u16_le(if self.darken_screen { 1 } else { 0 });
                stream.put_var_u32(self.color);
                stream.put_var_u32(self.overlay);
            }
            BossEvent::TYPE_TEXTURE => {
                stream.put_var_u32(self.color);
                stream.put_var_u32(self.overlay);
            }
            BossEvent::TYPE_HEALTH_PERCENT => {
                stream.put_f32_le(self.health_percent);
            }
            BossEvent::TYPE_TITLE => {
                PacketSerializer::put_string(&mut stream, self.title.clone());
                PacketSerializer::put_string(&mut stream, self.filtered_title.clone());
            }
            _ => {}
        }

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> BossEvent {
        let boss_actor_unique_id = PacketSerializer::get_actor_unique_id(stream);
        let event_type = stream.get_var_u32();
        let mut player_actor_unique_id = 0;
        let mut health_percent = 0.0;
        let mut title = String::new();
        let mut filtered_title = String::new();
        let mut darken_screen = false;
        let mut color = 0;
        let mut overlay = 0;

        match event_type {
            BossEvent::TYPE_REGISTER_PLAYER | BossEvent::TYPE_UNREGISTER_PLAYER | BossEvent::TYPE_QUERY => {
                player_actor_unique_id = PacketSerializer::get_actor_unique_id(stream);
            }
            BossEvent::TYPE_SHOW => {
                title = PacketSerializer::get_string(stream);
                filtered_title = PacketSerializer::get_string(stream);
                health_percent = stream.get_f32_le();

                // fallthrough: PROPERTIES
                let raw = stream.get_u16_le();
                darken_screen = if raw == 0 { false } else { true};

                // fallthrough: TEXTURE
                color = stream.get_var_u32();
                overlay = stream.get_var_u32();
            }
            BossEvent::TYPE_PROPERTIES => {
                let raw = stream.get_u16_le();
                darken_screen = if raw == 0 { false } else { true};

                // fallthrough: TEXTURE
                color = stream.get_var_u32();
                overlay = stream.get_var_u32();
            }
            BossEvent::TYPE_TEXTURE => {
                color = stream.get_var_u32();
                overlay = stream.get_var_u32();
            }
            BossEvent::TYPE_HEALTH_PERCENT => {
                health_percent = stream.get_f32_le();
            }
            BossEvent::TYPE_TITLE => {
                title = PacketSerializer::get_string(stream);
                filtered_title = PacketSerializer::get_string(stream);
            }
            _ => {}
        }

        BossEvent { boss_actor_unique_id, event_type, player_actor_unique_id, health_percent, title, filtered_title, darken_screen, color, overlay }
    }

    fn debug(&self) {
        println!("Boss Actor Unique ID: {}", self.boss_actor_unique_id);
        println!("Event Type: {}", self.event_type);
        println!("Player Actor Unique ID: {}", self.player_actor_unique_id);
        println!("Health Percent: {}", self.health_percent);
        println!("Title: {}", self.title);
        println!("Filtered Title: {}", self.filtered_title);
        println!("Darken Screen: {}", self.darken_screen);
        println!("Color: {}", self.color);
        println!("Overlay: {}", self.overlay);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl BossEvent {
    /** S2C: Shows the boss-bar to the player. */
    pub const TYPE_SHOW: u32 = 0;
    /** C2S: Registers a player to a boss fight. */
    pub const TYPE_REGISTER_PLAYER: u32 = 1;
    /** S2C: Removes the boss-bar from the client. */
    pub const TYPE_HIDE: u32 = 2;
    /** C2S: Unregisters a player from a boss fight. */
    pub const TYPE_UNREGISTER_PLAYER: u32 = 3;
    /** S2C: Sets the bar percentage. */
    pub const TYPE_HEALTH_PERCENT: u32 = 4;
    /** S2C: Sets the title of the bar. */
    pub const TYPE_TITLE: u32 = 5;
    /** S2C: Updates misc properties of the bar and environment. */
    pub const TYPE_PROPERTIES: u32 = 6;
    /** S2C: Updates boss-bar color and overlay texture. */
    pub const TYPE_TEXTURE: u32 = 7;
    /** C2S: Client asking the server to resend all boss data. */
    pub const TYPE_QUERY: u32 = 8;
}
