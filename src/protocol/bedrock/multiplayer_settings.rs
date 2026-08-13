use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct MultiplayerSettings {
    pub action: i32,
}

impl Packet for MultiplayerSettings {
    fn id(&self) -> u16 {
        BedrockPacketType::IDMultiplayerSettings.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_var_i32(self.action);
    }

    fn decode(stream: &mut Reader) -> MultiplayerSettings {
        let action = stream.get_var_i32();

        MultiplayerSettings { action }
    }
}

impl MultiplayerSettings {
    pub const ACTION_ENABLE_MULTIPLAYER: i32 = 0;
    pub const ACTION_DISABLE_MULTIPLAYER: i32 = 1;
    pub const ACTION_REFRESH_JOIN_CODE: i32 = 2;
}
