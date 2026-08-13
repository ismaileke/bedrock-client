use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct UpdateAdventureSettings {
    pub no_attacking_mobs: bool,
    pub no_attacking_players: bool,
    pub world_immutable: bool,
    pub show_name_tags: bool,
    pub auto_jump: bool,
}

impl Packet for UpdateAdventureSettings {
    fn id(&self) -> u16 {
        BedrockPacketType::IDUpdateAdventureSettings.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_bool(self.no_attacking_mobs);
        stream.put_bool(self.no_attacking_players);
        stream.put_bool(self.world_immutable);
        stream.put_bool(self.show_name_tags);
        stream.put_bool(self.auto_jump);
    }

    fn decode(stream: &mut Reader) -> UpdateAdventureSettings {
        let no_attacking_mobs = stream.get_bool();
        let no_attacking_players = stream.get_bool();
        let world_immutable = stream.get_bool();
        let show_name_tags = stream.get_bool();
        let auto_jump = stream.get_bool();

        UpdateAdventureSettings {
            no_attacking_mobs,
            no_attacking_players,
            world_immutable,
            show_name_tags,
            auto_jump,
        }
    }
}
