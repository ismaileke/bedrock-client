use binary_utils::binary::{Reader, Writer};
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

#[derive(serde::Serialize, Debug)]
pub struct MatchmakingStateOptions {
    pub triggering_player_name: Option<String>,
    pub triggered_by_local_player: Option<bool>
}

impl MatchmakingStateOptions {
    pub fn new(triggering_player_name: Option<String>, triggered_by_local_player: Option<bool>) -> MatchmakingStateOptions {
        MatchmakingStateOptions { triggering_player_name, triggered_by_local_player }
    }

    pub fn read(stream: &mut Reader) -> MatchmakingStateOptions {
        let triggering_player_name = PacketSerializer::read_optional(stream, |s| PacketSerializer::get_string(s));
        let triggered_by_local_player = PacketSerializer::read_optional(stream, |s| s.get_bool());

        MatchmakingStateOptions { triggering_player_name, triggered_by_local_player }
    }

    pub fn write(&self, stream: &mut Writer) {
        PacketSerializer::write_optional(stream, &self.triggering_player_name, |s, v| PacketSerializer::put_string(s, v));
        PacketSerializer::write_optional(stream, &self.triggered_by_local_player, |s, v| s.put_bool(*v));
    }
}