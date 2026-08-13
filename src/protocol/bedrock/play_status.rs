use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::{Reader, Writer};
use std::convert::TryFrom;

#[repr(u32)]
pub enum LoginStatus {
    LoginSuccess = 0,
    LoginFailedClient,
    LoginFailedServer,
    PlayerSpawn,
    LoginFailedInvalidTenant,
    LoginFailedVanillaEdu,
    LoginFailedEduVanilla,
    LoginFailedServerFull,
    LoginFailedEditorVanilla,
    LoginFailedVanillaEditor,
}

impl TryFrom<u32> for LoginStatus {
    type Error = &'static str;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(LoginStatus::LoginSuccess),
            1 => Ok(LoginStatus::LoginFailedClient),
            2 => Ok(LoginStatus::LoginFailedServer),
            3 => Ok(LoginStatus::PlayerSpawn),
            4 => Ok(LoginStatus::LoginFailedInvalidTenant),
            5 => Ok(LoginStatus::LoginFailedVanillaEdu),
            6 => Ok(LoginStatus::LoginFailedEduVanilla),
            7 => Ok(LoginStatus::LoginFailedServerFull),
            8 => Ok(LoginStatus::LoginFailedEditorVanilla),
            9 => Ok(LoginStatus::LoginFailedVanillaEditor),
            _ => Err("Invalid status value"),
        }
    }
}

#[derive(serde::Serialize, Debug)]
pub struct PlayStatus {
    pub status: u32,
}

impl Packet for PlayStatus {
    fn id(&self) -> u16 {
        BedrockPacketType::IDPlayStatus.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_u32_be(self.status);
    }

    fn decode(stream: &mut Reader) -> PlayStatus {
        PlayStatus { status: stream.get_u32_be(), }
    }
}
