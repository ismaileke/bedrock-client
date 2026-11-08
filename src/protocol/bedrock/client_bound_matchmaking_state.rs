use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};
use crate::protocol::bedrock::types::matchmaking_state_options::MatchmakingStateOptions;

#[derive(serde::Serialize, Debug)]
pub struct ClientBoundMatchmakingState {
    pub state: u8,
    pub destination_name: String,
    pub options: Option<MatchmakingStateOptions>
}

impl ClientBoundMatchmakingState {
    pub const MATCHMAKING_STATE_IDLE: u8 = 0;
    pub const MATCHMAKING_STATE_MATCHMAKING: u8 = 1;
    pub const MATCHMAKING_STATE_MATCH_FOUND: u8 = 2;
    pub const MATCHMAKING_STATE_CANCELED: u8 = 3;
    pub const MATCHMAKING_STATE_PLAYER_LEFT_PARTY: u8 = 4;
    pub const MATCHMAKING_STATE_PLAYER_LEFT_SERVER: u8 = 5;
    pub const MATCHMAKING_STATE_SERVER_SHUTDOWN: u8 = 6;
    pub const MATCHMAKING_STATE_TIMED_OUT: u8 = 7;
    pub const MATCHMAKING_STATE_REQUEUE_AS_PARTY: u8 = 8;
}

impl Packet for ClientBoundMatchmakingState {
    fn id(&self) -> u16 {
        BedrockPacketType::IDClientBoundMatchmakingState.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_u8(self.state);
        PacketSerializer::put_string(stream, &self.destination_name);
        PacketSerializer::write_optional(stream, &self.options, |s, v| v.write(s))
    }

    fn decode(stream: &mut Reader) -> ClientBoundMatchmakingState {
        let state = stream.get_u8();
        let destination_name = PacketSerializer::get_string(stream);
        let options = PacketSerializer::read_optional(stream, |s| MatchmakingStateOptions::read(s));
        ClientBoundMatchmakingState { state, destination_name, options }
    }
}
